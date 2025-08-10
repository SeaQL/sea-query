#[path = "../../src/token.rs"]
mod token;
use token::*;

use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    Ident, LitStr, Token,
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

    let mut expand_array = true;

    let backend = match backend.to_string().as_str() {
        "postgres" => {
            expand_array = false;
            quote!(sea_query::PostgresQueryBuilder)
        }
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
    let mut fragment = String::new();
    let mut vars: Vec<&str> = Default::default();
    let mut paren_depth = 0;

    for token in tokens {
        match token {
            Token::Punctuation("{") => {
                in_brace = true;
            }
            Token::Punctuation("}") => {
                assert!(in_brace, "Non-matching closing brace }}");
                let mut var = TokenStream::new();
                for (i, v) in vars.iter().enumerate() {
                    if i > 0 {
                        var.extend(quote!(.));
                    }
                    let v = Ident::new(v, Span::call_site());
                    var.extend(quote!(#v));
                }
                // only expand when surrounded by parenthesis `({a})`
                // or there is a spread operator `{..a}`
                if (expand_array && paren_depth > 0) || dot_count == 2 {
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
                vars.clear();
            }
            Token::Unquoted(var) if in_brace => {
                if !fragment.is_empty() {
                    fragments.push(quote!(.push_fragment(#fragment)));
                    fragment.clear();
                }
                vars.push(var);
            }
            Token::Punctuation(".") if in_brace => {
                if vars.is_empty() {
                    // prefix ..
                    dot_count += 1;
                }
            }
            Token::Punctuation("(") => {
                paren_depth += 1;
                fragment.push_str(token.as_str());
            }
            Token::Punctuation(")") => {
                paren_depth -= 1;
                fragment.push_str(token.as_str());
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
        #(#params)*;

        query
    }};

    Ok(output)
}
