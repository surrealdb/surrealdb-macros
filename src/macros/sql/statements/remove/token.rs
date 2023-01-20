use super::Base;
use crate::macros::sql::kw;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;

enum BaseOrScope {
	Base(Base),
	Sc(Ident),
}

impl Parse for BaseOrScope {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"NAMESPACE" | "NS" => Ok(Self::Base(Base::Ns)),
			"DATABASE" | "DB" => Ok(Self::Base(Base::Db)),
			"SCOPE" => Ok(Self::Sc(input.parse()?)),
			_ => {
				let message = format!(
					"expected one of `NAMESPACE`, `NS`, `DATABASE`, `DB`, `SCOPE`, found `{token}`"
				);
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for BaseOrScope {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match &self {
			BaseOrScope::Base(base) => tokens.append_all(quote!(#base)),
			BaseOrScope::Sc(scope) => {
				let scope = scope.to_string();
				tokens.append_all(quote!(Base::Sc(#scope.to_owned().into())));
			}
		}
	}
}

pub struct Statement {
	name: Ident,
	_on: kw::On,
	base: BaseOrScope,
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
		tokens.append_all(quote!(RemoveTokenStatement {
			name: #name.to_owned().into(),
			base: #base,
		}));
	}
}
