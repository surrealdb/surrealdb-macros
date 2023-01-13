use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn option_statement() {
	let query = sql!(OPTION IMPORT);
	let expected = parse("OPTION IMPORT").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn option_statement_true() {
	let query = sql!(OPTION IMPORT = TRUE);
	let expected = parse("OPTION IMPORT = TRUE").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn option_statement_false() {
	let query = sql!(OPTION IMPORT = FALSE);
	let expected = parse("OPTION IMPORT = FALSE").unwrap();
	assert_eq!(query, expected);
}
