use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;

pub struct Statement {
	ns: String,
	db: String,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut statement = Statement {
			ns: String::new(),
			db: String::new(),
		};
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"NAMESPACE" | "NS" => {
				statement.ns = input.parse::<Ident>()?.to_string();
				let token: Option<Ident> = input.parse()?;
				if let Some(token) = token {
					match uppercase!(token).as_str() {
						"DATABASE" | "DB" => {
							statement.db = input.parse::<Ident>()?.to_string();
						}
						_ => {
							let message = format!("expected `DATABASE` or `DB`, found `{token}`");
							return Err(Error::new_spanned(token, message));
						}
					}
				}
			}
			"DATABASE" | "DB" => {
				statement.db = input.parse::<Ident>()?.to_string();
			}
			_ => {
				let message = format!(
					"expected one of `NAMESPACE, `NS`, `DATABASE` or `DB`, found `{token}`"
				);
				return Err(Error::new_spanned(token, message));
			}
		}
		Ok(statement)
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let Statement {
			ns,
			db,
		} = self;
		let namespace = if ns.is_empty() {
			quote!(None)
		} else {
			quote!(Some(#ns.to_owned()))
		};
		let database = if db.is_empty() {
			quote!(None)
		} else {
			quote!(Some(#db.to_owned()))
		};
		tokens.append_all(quote!(UseStatement {
			ns: #namespace,
			db: #database,
		}));
	}
}
