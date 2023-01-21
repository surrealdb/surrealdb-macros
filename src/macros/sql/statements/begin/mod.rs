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
	_transaction: Option<kw::Transaction>,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Option<Ident> = input.parse()?;
		let _transaction = match token {
			Some(token) => Some(token.try_into()?),
			None => None,
		};
		Ok(Self {
			_transaction,
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(BeginStatement));
	}
}
