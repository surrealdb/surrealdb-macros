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
	name: Ident,
	_equal: Option<Token![=]>,
	what: bool,
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let name = input.parse()?;
		let Some(_equal) = input.parse()? else {
			return Ok(Self {
				name,
				_equal: None,
				what: true,
			});
		};
		let token: Ident = input.parse()?;
		let what = match uppercase!(token).as_str() {
			"TRUE" => true,
			"FALSE" => false,
			_ => {
				let message = format!("expected `TRUE` or `FALSE`, found `{token}`");
				return Err(Error::new_spanned(token, message));
			}
		};
		Ok(Self {
			name,
			what,
			_equal: Some(_equal),
		})
	}
}

impl ToTokens for Statement {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name.to_string();
		let what = self.what;
		tokens.append_all(quote!(OptionStatement {
			name: #name.to_owned().into(),
			what: #what,
		}));
	}
}
