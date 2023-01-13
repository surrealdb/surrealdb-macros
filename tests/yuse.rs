use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn use_query_ns() {
	let query = sql!(USE NS test);
	let expected = parse("USE NS test").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn use_query_db() {
	let query = sql!(USE DB test);
	let expected = parse("USE DB test").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn use_query_both() {
	let query = sql!(USE NS test DB test);
	let expected = parse("USE NS test DB test").unwrap();
	assert_eq!(query, expected);
}
