use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn commit_basic() {
	let query = sql!(COMMIT);
	let expected = parse("COMMIT").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn commit_query() {
	let query = sql!(COMMIT TRANSACTION);
	let expected = parse("COMMIT TRANSACTION").unwrap();
	assert_eq!(query, expected);
}
