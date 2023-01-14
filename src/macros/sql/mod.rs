mod query;
mod statements;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

pub(crate) fn parse(input: TokenStream) -> TokenStream {
	let query = parse_macro_input!(input as query::Query);
	quote!({
		use surrealdb::sql::*;
		use surrealdb::sql::statements::*;

		#query
	})
	.into()
}
