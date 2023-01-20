macro_rules! uppercase {
	($token:expr) => {
		$token.to_string().to_ascii_uppercase()
	};
}

macro_rules! keyword {
	($keyword:ident) => {
		pub(super) struct $keyword(syn::Ident);

		impl TryFrom<syn::Ident> for $keyword {
			type Error = syn::Error;

			fn try_from(token: syn::Ident) -> syn::parse::Result<Self> {
				let keyword = uppercase!(stringify!($keyword));
				if uppercase!(token) != keyword {
					let message = format!("expected `{keyword}`, found `{token}`");
					return Err(syn::Error::new_spanned(token, message));
				}
				Ok(Self(token))
			}
		}

		impl syn::parse::Parse for $keyword {
			fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
				input.parse::<syn::Ident>()?.try_into()
			}
		}

		impl quote::ToTokens for $keyword {
			fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
				self.0.to_tokens(tokens);
			}
		}

		impl std::fmt::Display for $keyword {
			fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				self.0.fmt(formatter)
			}
		}
	};
}

#[allow(clippy::used_underscore_binding)]
pub(super) mod sql;
