use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn cancel_basic() {
	let query = sql!(CANCEL);
	let expected = parse("CANCEL").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn cancel_query() {
	let query = sql!(CANCEL TRANSACTION);
	let expected = parse("CANCEL TRANSACTION").unwrap();
	assert_eq!(query, expected);
}
