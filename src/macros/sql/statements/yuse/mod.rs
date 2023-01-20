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
	ns: Option<Ident>,
	db: Option<Ident>,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let mut statement = Statement {
			ns: None,
			db: None,
		};
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"NAMESPACE" | "NS" => {
				statement.ns = Some(input.parse()?);
				let token: Option<Ident> = input.parse()?;
				if let Some(token) = token {
					match uppercase!(token).as_str() {
						"DATABASE" | "DB" => {
							statement.db = Some(input.parse()?);
						}
						_ => {
							let message = format!("expected `DATABASE` or `DB`, found `{token}`");
							return Err(Error::new_spanned(token, message));
						}
					}
				}
			}
			"DATABASE" | "DB" => {
				statement.db = Some(input.parse()?);
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
		let namespace = match &self.ns {
			Some(ns) => {
				let ns = ns.to_string();
				quote!(Some(#ns.to_owned()))
			}
			None => quote!(None),
		};
		let database = match &self.db {
			Some(db) => {
				let db = db.to_string();
				quote!(Some(#db.to_owned()))
			}
			None => quote!(None),
		};
		tokens.append_all(quote!(UseStatement {
			ns: #namespace,
			db: #database,
		}));
	}
}
