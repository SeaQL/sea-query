#[path = "../../src/token.rs"]
mod token;
use token::*;

use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Ident, Index, LitStr, Member, Token,
    parse::{Parse, ParseStream},
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
    let tokens = Tokenizer::new(&sql_input);
    let mut in_brace = false;
    let mut in_paren = false;
    let mut dot_count = 0;
    let mut interpolate = false;
    let mut nested_eval = false;
    let mut has_ending_comma = false;
    let mut fragment = String::new();
    let mut vars: Vec<Var> = vec![Default::default()];

    #[derive(Default)]
    struct Var<'a> {
        parts: Vec<&'a str>, // named parts, i.e. a.b.c
        members: Vec<u32>,   // tuple members
    }

    for token in tokens {
        match token {
            Token::Punctuation("{") => {
                in_brace = true;
            }
            Token::Punctuation("}") => {
                assert!(in_brace, "unmatched closing brace }}");

                for vars in vars.iter_mut() {
                    if interpolate {
                        assert!(vars.members.len() >= 2, "expect 2 numbers around :");
                        let a = vars.members[vars.members.len() - 2];
                        let b = vars.members[vars.members.len() - 1];
                        assert!(a < b, "expect a < b in a:b");
                        vars.members = (a..=b).collect();
                    } else {
                        vars.members.clear();
                    }
                }

                let top = {
                    let v = Ident::new(vars[0].parts[0], Span::call_site());
                    quote!(#v)
                };
                if nested_eval {
                    assert!(has_ending_comma, "..(), must end with comma ,");
                    let group_size: usize = vars
                        .iter()
                        .map(|var| {
                            if var.members.is_empty() {
                                1
                            } else {
                                var.members.len()
                            }
                        })
                        .sum();
                    fragments
                        .push(quote!(.push_tuple_parameter_groups((&#top).p_len(), #group_size)));
                }
                let mut group = Vec::new();

                for vars in vars {
                    let mut var = TokenStream::new();
                    for (i, v) in vars.parts.iter().enumerate() {
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
                    if !vars.members.is_empty() {
                        // there is a range operator `a:b`
                        if !nested_eval {
                            let len = vars.members.len();
                            fragments.push(quote!(.push_parameters(#len)));
                        }

                        for mul in vars.members.iter() {
                            let mut var = var.clone();
                            let mul = Member::Unnamed(Index {
                                index: *mul,
                                span: Span::call_site(),
                            });
                            var.extend(quote!(#mul));
                            group.push(quote! { query = query.bind(&#var); });
                        }
                    } else if dot_count == 2 && !nested_eval {
                        // non nested spread `..a`
                        fragments.push(quote!(.push_parameters((&#var).p_len())));
                        group.push(quote! {
                            for v in (&#var).iter_p().iter() {
                                query = query.bind(v);
                            }
                        });
                    } else {
                        if !nested_eval {
                            fragments.push(quote!(.push_parameters(1)));
                        }
                        group.push(quote! { query = query.bind(&#var); });
                    }
                }

                if nested_eval {
                    params.push(quote! {
                        for #top in (&#top).iter_p().iter() {
                            #(#group)*
                        }
                    });
                } else {
                    params.append(&mut group);
                }

                in_brace = false;
                in_paren = false;
                dot_count = 0;
                interpolate = false;
                nested_eval = false;
                has_ending_comma = false;
                vars = vec![Default::default()];
            }
            Token::Unquoted(var) if in_brace => {
                if !fragment.is_empty() {
                    fragments.push(quote!(.push_fragment(#fragment)));
                    fragment.clear();
                }
                vars.last_mut().unwrap().parts.push(var);
                if is_ascii_digits(var) {
                    vars.last_mut()
                        .unwrap()
                        .members
                        .push(var.parse().expect("index out of range"));
                }
            }
            Token::Punctuation(".") if in_brace => {
                if vars.last_mut().unwrap().parts.is_empty() {
                    // prefix ..
                    dot_count += 1;
                }
            }
            Token::Punctuation(":") if in_brace => {
                if !vars.last_mut().unwrap().parts.is_empty() {
                    // postfix :
                    interpolate = true;
                }
            }
            Token::Punctuation("(") if in_brace => {
                nested_eval = true;
                in_paren = true;
            }
            Token::Punctuation(")") if in_brace => {
                assert!(in_paren, "unmatched closing parenthesis )");
                in_paren = false
            }
            Token::Punctuation(",") if in_brace && in_paren && nested_eval => {
                // push a new variable
                vars.push(Default::default());
            }
            Token::Punctuation(",") if in_brace && !in_paren && nested_eval => {
                has_ending_comma = true;
            }
            Token::Punctuation(",") if in_brace && !in_paren && !nested_eval => {
                panic!("unknown extra comma ,")
            }
            _ => {
                if !in_brace {
                    fragment.push_str(token.as_str());
                }
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
