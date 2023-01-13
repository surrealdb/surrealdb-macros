use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn kill_query() {
	let query = sql!(KILL "3589ffe7-0a68-4f5f-998c-728e77586ed2");
	let expected = parse("KILL '3589ffe7-0a68-4f5f-998c-728e77586ed2'").unwrap();
	assert_eq!(query, expected);
}
