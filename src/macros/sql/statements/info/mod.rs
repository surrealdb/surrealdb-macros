use crate::macros::sql::kw;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;

enum Item {
	Kv,
	Ns,
	Db,
	Sc(Ident),
	Tb(Ident),
}

impl Parse for Item {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"KV" => Ok(Item::Kv),
			"NAMESPACE" | "NS" => Ok(Item::Ns),
			"DATABASE" | "DB" => Ok(Item::Db),
			"SCOPE" | "SC" => Ok(Item::Sc(input.parse()?)),
			"TABLE" | "TB" => Ok(Item::Tb(input.parse()?)),
			_ => {
				let message = format!("expected one of `KV`, `NAMESPACE`, `NS`, `DATABASE`, `DB`, `SCOPE`, `SC`, `TABLE` or `TB`, found `{token}`");
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for Item {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Item::Kv => tokens.append_all(quote!(InfoStatement::Kv)),
			Item::Ns => tokens.append_all(quote!(InfoStatement::Ns)),
			Item::Db => tokens.append_all(quote!(InfoStatement::Db)),
			Item::Sc(scope) => {
				let scope = scope.to_string();
				tokens.append_all(quote!(InfoStatement::Sc(#scope.to_owned().into())));
			}
			Item::Tb(table) => {
				let table = table.to_string();
				tokens.append_all(quote!(InfoStatement::Tb(#table.to_owned().into())));
			}
		};
	}
}

pub struct Statement {
	_for: kw::For,
	item: Item,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			_for: input.parse()?,
			item: input.parse()?,
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		self.item.to_tokens(tokens);
	}
}
