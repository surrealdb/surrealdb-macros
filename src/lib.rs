extern crate proc_macro;

#[macro_use]
mod macros;

use proc_macro::TokenStream;

/// Parses a set of `SurrealQL` statements
///
/// # Examples
///
/// ```no_run
/// # use surrealdb_macros::sql;
/// let query = sql!(REMOVE FIELD bar ON TABLE foo);
/// ```
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
	macros::sql::parse(input)
}
