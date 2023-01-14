use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::Error;
use syn::Ident;

pub enum Statement {
	Kv,
	Ns,
	Db,
	Sc(String),
	Tb(String),
}

impl Parse for Statement {
	fn parse(input: ParseStream) -> Result<Self> {
		let token: Ident = input.parse()?;
		let expected = "FOR";
		if uppercase!(token).as_str() != expected {
			let message = format!("expected `{expected}`, found `{token}`");
			return Err(Error::new_spanned(token, message));
		}
		let token: Ident = input.parse()?;
		match uppercase!(token).as_str() {
			"KV" => Ok(Self::Kv),
			"NAMESPACE" | "NS" => Ok(Self::Ns),
			"DATABASE" | "DB" => Ok(Self::Db),
			"SCOPE" | "SC" => {
				let token: Ident = input.parse()?;
				Ok(Self::Sc(token.to_string()))
			}
			"TABLE" | "TB" => {
				let token: Ident = input.parse()?;
				Ok(Self::Tb(token.to_string()))
			}
			_ => {
				let message = format!("expected one of `KV`, `NAMESPACE`, `NS`, `DATABASE`, `DB`, `SCOPE`, `SC`, `TABLE` or `TB`, found `{token}`");
				Err(Error::new_spanned(token, message))
			}
		}
	}
}

impl ToTokens for Statement {
	#[rustfmt::skip]
	fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Statement::Kv => tokens.append_all(quote!(InfoStatement::Kv)),
            Statement::Ns => tokens.append_all(quote!(InfoStatement::Ns)),
            Statement::Db => tokens.append_all(quote!(InfoStatement::Db)),
            Statement::Sc(scope) => tokens.append_all(quote!(InfoStatement::Sc(#scope.to_owned().into()))),
            Statement::Tb(table) => tokens.append_all(quote!(InfoStatement::Tb(#table.to_owned().into()))),
        };
	}
}
