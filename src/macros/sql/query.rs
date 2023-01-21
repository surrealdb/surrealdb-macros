use super::statements::Statement;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;
use syn::punctuated::Punctuated;
use syn::Error;
use syn::Token;

pub(super) struct Query {
	statements: Punctuated<Statement, Token![;]>,
}

impl Parse for Query {
	fn parse(input: ParseStream) -> Result<Self> {
		if input.is_empty() {
			return Err(Error::new(Span::call_site(), "expected SQL code"));
		}
		Ok(Query {
			statements: input.parse_terminated(Statement::parse)?,
		})
	}
}

impl ToTokens for Query {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let len = self.statements.len();
		let mut statements = quote! {
			let mut statements = Vec::with_capacity(#len);
		};
		for statement in &self.statements {
			statements.append_all(quote! {
				statements.push(#statement);
			});
		}
		statements.append_all(quote!(statements));
		tokens.append_all(quote!(Query(Statements({#statements}))));
	}
}
