use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;
use syn::Token;

pub struct Statement {
	name: String,
	what: bool,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		let name = token.to_string();
		let Some(_): Option<Token![=]> = input.parse()? else {
            return Ok(Self { name, what: true });
        };
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"TRUE" => Ok(Self {
				name,
				what: true,
			}),
			"FALSE" => Ok(Self {
				name,
				what: false,
			}),
			_ => {
				let message = format!("expected `TRUE` or `FALSE`, found `{token}`");
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		let what = self.what;
		tokens.append_all(quote!(OptionStatement {
			name: #name.to_owned().into(),
			what: #what,
		}));
	}
}
