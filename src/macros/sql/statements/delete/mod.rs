use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;

pub struct Statement;

impl Parse for Statement {
	fn parse(_input: ParseStream) -> Result<Self> {
		todo!()
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, _tokens: &mut TokenStream) {
		todo!()
	}
}
