use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::LitStr;
use uuid::Uuid;

pub struct Statement {
	id: String,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: LitStr = input.parse()?;
		let id = token.value();
		if Uuid::parse_str(&id).is_err() {
			let message = format!("expected a UUID, found `{id}`");
			return Err(Error::new_spanned(token, message));
		}
		Ok(Self {
			id,
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let id = &self.id;
		tokens.append_all(quote!(KillStatement {
			id: #id.into(),
		}));
	}
}
