use super::Base;
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
	base: Base,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			name: input.parse()?,
			_on: input.parse()?,
			base: input.parse()?,
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = self.name.to_string();
		let base = &self.base;
		tokens.append_all(quote!(RemoveLoginStatement {
			name: #name.to_owned().into(),
			base: #base,
		}));
	}
}
