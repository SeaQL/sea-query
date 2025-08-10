#[path = "../../src/token.rs"]
mod token;
use token::*;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    Ident, Index, LitStr, Member, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

struct CallArgs {
    module: Ident,
    backend: Ident,
    method: Ident,
    sql_holder: Option<Ident>,
    sql_input: LitStr,
}

impl Parse for CallArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let module: Ident = input.parse()?;
        let _colon1: Token![::] = input.parse()?;
        let backend: Ident = input.parse()?;
        let _colon2: Token![::] = input.parse()?;
        let method: Ident = input.parse()?;
        let _comma: Token![,] = input.parse()?;
        let sql_holder = if input.peek(Ident) {
            let ident = input.parse()?;
            let _assign: Token![=] = input.parse()?;
            Some(ident)
        } else {
            None
        };
        let sql_input: LitStr = input.parse()?;

        Ok(CallArgs {
            module,
            backend,
            method,
            sql_holder,
            sql_input,
        })
    }
}

pub fn expand(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    let CallArgs {
        module,
        backend,
        method,
        sql_holder,
        sql_input,
    } = syn::parse(input)?;

    let backend = match backend.to_string().as_str() {
        "postgres" => quote!(sea_query::PostgresQueryBuilder),
        "mysql" => quote!(sea_query::MysqlQueryBuilder),
        "sqlite" => quote!(sea_query::SqliteQueryBuilder),
        _ => quote!(#backend),
    };

    let mut fragments = Vec::new();
    let mut params = Vec::new();

    let sql_input = sql_input.value();
    let mut tokens = Tokenizer::new(&sql_input);
    let mut in_brace = false;
    let mut dot_count = 0;
    let mut interpolate = false;
    let mut nested_group = false;
    let mut has_comma = false;
    let mut fragment = String::new();
    let mut vars: Vec<&str> = Default::default();
    let mut muls: Vec<u32> = Default::default();

    for token in tokens {
        match token {
            Token::Punctuation("{") => {
                in_brace = true;
            }
            Token::Punctuation("}") => {
                assert!(in_brace, "unmatched closing brace }}");

                if interpolate {
                    assert!(muls.len() >= 2, "expect 2 numbers around :");
                    let a = muls[muls.len() - 2];
                    let b = muls[muls.len() - 1];
                    assert!(a < b, "expect a < b in a:b");
                    muls = (a..=b).collect();
                } else {
                    muls.clear();
                }

                let mut var = TokenStream::new();
                for (i, v) in vars.iter().enumerate() {
                    if i > 0 {
                        var.extend(quote!(.));
                    }
                    if is_ascii_digits(v) {
                        if interpolate {
                            // skip .x
                            break;
                        }
                        let v = Member::Unnamed(Index {
                            index: v.parse().unwrap(),
                            span: Span::call_site(),
                        });
                        var.extend(quote!(#v));
                    } else {
                        let v = Ident::new(v, Span::call_site());
                        var.extend(quote!(#v));
                    }
                }
                if !muls.is_empty() {
                    // there is a range operator `a:b`
                    let top = {
                        let v = Ident::new(vars[0], Span::call_site());
                        quote!(#v)
                    };
                    let len = muls.len();
                    if nested_group {
                        assert!(has_comma, "..(), must end with comma ,");
                        fragments.push(quote!(.push_tuple_parameter_groups((&#top).p_len(), #len)));
                    } else {
                        fragments.push(quote!(.push_parameters(#len)));
                    }

                    let mut group = Vec::new();
                    for (j, mul) in muls.iter().enumerate() {
                        let mut var = var.clone();
                        let mul = Member::Unnamed(Index {
                            index: *mul,
                            span: Span::call_site(),
                        });
                        var.extend(quote!(#mul));
                        group.push(quote! { query = query.bind(&#var); });
                    }

                    if nested_group {
                        params.push(quote! {
                            for #top in (&#top).iter_p().iter() {
                                #(#group)*
                            }
                        });
                    } else {
                        params.append(&mut group);
                    }
                } else if dot_count == 2 {
                    // there is a spread operator `{..a}`
                    fragments.push(quote!(.push_parameters((&#var).p_len())));
                    params.push(quote! {
                        for v in (&#var).iter_p().iter() {
                            query = query.bind(v);
                        }
                    });
                } else {
                    fragments.push(quote!(.push_parameters(1)));
                    params.push(quote! { query = query.bind(&#var); });
                }

                in_brace = false;
                dot_count = 0;
                interpolate = false;
                nested_group = false;
                has_comma = false;
                vars.clear();
                muls.clear();
            }
            Token::Unquoted(var) if in_brace => {
                if !fragment.is_empty() {
                    fragments.push(quote!(.push_fragment(#fragment)));
                    fragment.clear();
                }
                vars.push(var);
                if is_ascii_digits(var) {
                    muls.push(var.parse().expect("index out of range"));
                }
            }
            Token::Punctuation(".") if in_brace => {
                if vars.is_empty() {
                    // prefix ..
                    dot_count += 1;
                }
            }
            Token::Punctuation(":") if in_brace => {
                if !vars.is_empty() {
                    // postfix :
                    interpolate = true;
                }
            }
            Token::Punctuation("(") if in_brace => {
                nested_group = true;
            }
            Token::Punctuation(")") if in_brace => {
                assert!(nested_group, "unmatched closing parenthesis )")
            }
            Token::Punctuation(",") if in_brace && nested_group => {
                has_comma = true;
            }
            _ => {
                fragment.push_str(token.as_str());
            }
        }
    }
    if !fragment.is_empty() {
        fragments.push(quote!(.push_fragment(#fragment)));
        fragment.clear();
    }

    let (maybe_let, sql_holder) = if let Some(sql_holder) = sql_holder {
        (quote!(), sql_holder)
    } else {
        (quote!(let), Ident::new("sql", Span::call_site()))
    };

    let output = quote! {{
        use sea_query::raw_sql::*;
        let mut builder = RawSqlQueryBuilder::new(#backend);
        builder
            #(#fragments)*;

        #maybe_let #sql_holder = builder.finish();
        let mut query = #module::#method(&#sql_holder);
        #(#params)*

        query
    }};

    Ok(output)
}

fn is_ascii_digits(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}
