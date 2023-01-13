use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;

pub struct Statement;

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Option<Ident> = input.parse()?;
		if let Some(token) = token {
			let expected = "TRANSACTION";
			if token.to_string().to_ascii_uppercase() != expected {
				let message = format!("Unknown statement `{token}`, did you mean `{expected}`?");
				return Err(Error::new_spanned(token, message));
			}
		}
		Ok(Self)
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		tokens.append_all(quote!(CancelStatement));
	}
}
