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
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		Ok(Self {
			name: input.parse()?,
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = self.name.to_string();
		tokens.append_all(quote!(RemoveParamStatement {
			name: #name.to_owned().into(),
		}));
	}
}
