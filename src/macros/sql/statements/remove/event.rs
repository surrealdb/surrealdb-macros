use crate::macros::sql::kw;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Ident;

pub struct Statement {
	name: Ident,
	_on: kw::On,
	table: Ident,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			name: input.parse()?,
			_on: input.parse()?,
			table: {
				let token: Ident = input.parse()?;
				match uppercase!(token).as_str() {
					"TABLE" => input.parse()?,
					_ => token,
				}
			},
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = self.name.to_string();
		let table = self.table.to_string();
		tokens.append_all(quote!(RemoveEventStatement {
			name: #name.to_owned().into(),
			what: #table.to_owned().into(),
		}));
	}
}
