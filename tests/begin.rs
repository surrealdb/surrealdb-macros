use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn begin_basic() {
	let query = sql!(BEGIN);
	let expected = parse("BEGIN").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn begin_query() {
	let query = sql!(BEGIN TRANSACTION);
	let expected = parse("BEGIN TRANSACTION").unwrap();
	assert_eq!(query, expected);
}
