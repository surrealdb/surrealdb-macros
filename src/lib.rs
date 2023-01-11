extern crate proc_macro;

mod macros;

use proc_macro::TokenStream;

/// Parses a set of `SurrealQL` statements
///
/// # Examples
///
/// ```no_run
/// # use surrealdb_macros::sql;
/// let query = sql!(BEGIN TRANSACTION);
/// ```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
	macros::sql::parse(input)
}
