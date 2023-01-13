use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn info_query_ns() {
	let query = sql!(INFO FOR NAMESPACE);
	let expected = parse("INFO FOR NAMESPACE").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn info_query_db() {
	let query = sql!(INFO FOR DATABASE);
	let expected = parse("INFO FOR DATABASE").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn info_query_sc() {
	let query = sql!(INFO FOR SCOPE test);
	let expected = parse("INFO FOR SCOPE test").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn info_query_tb() {
	let query = sql!(INFO FOR TABLE test);
	let expected = parse("INFO FOR TABLE test").unwrap();
	assert_eq!(query, expected);
}
