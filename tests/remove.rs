use surrealdb::sql::parse;
use surrealdb_macros::sql;

#[test]
fn remove_namespace() {
	let query = sql!(REMOVE NAMESPACE name);
	let expected = parse("REMOVE NAMESPACE name").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_database() {
	let query = sql!(REMOVE DATABASE name);
	let expected = parse("REMOVE DATABASE name").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_user() {
	let query = sql!(REMOVE USER name ON NAMESPACE);
	let expected = parse("REMOVE USER name ON NAMESPACE").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_token() {
	let query = sql!(REMOVE TOKEN name ON NAMESPACE);
	let expected = parse("REMOVE TOKEN name ON NAMESPACE").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_scope() {
	let query = sql!(REMOVE SCOPE name);
	let expected = parse("REMOVE SCOPE name").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_param() {
	let query = sql!(REMOVE PARAM $name);
	let expected = parse("REMOVE PARAM $name").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_table() {
	let query = sql!(REMOVE TABLE name);
	let expected = parse("REMOVE TABLE name").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_event() {
	let query = sql!(REMOVE EVENT name ON what);
	let expected = parse("REMOVE EVENT name ON what").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_field() {
	let query = sql!(REMOVE FIELD name ON what);
	let expected = parse("REMOVE FIELD name ON what").unwrap();
	assert_eq!(query, expected);

	let query = sql!(REMOVE FIELD name ON TABLE what);
	let expected = parse("REMOVE FIELD name ON TABLE what").unwrap();
	assert_eq!(query, expected);

	let query = sql!(REMOVE FIELD name.* ON what);
	let expected = parse("REMOVE FIELD name.* ON what").unwrap();
	assert_eq!(query, expected);

	let query = sql!(REMOVE FIELD name[*] ON what);
	let expected = parse("REMOVE FIELD name[*] ON what").unwrap();
	assert_eq!(query, expected);

	let query = sql!(REMOVE FIELD name[1] ON what);
	let expected = parse("REMOVE FIELD name[1] ON what").unwrap();
	assert_eq!(query, expected);

	let query = sql!(REMOVE FIELD name.inner ON what);
	let expected = parse("REMOVE FIELD name.inner ON what").unwrap();
	assert_eq!(query, expected);

	let query = sql!(REMOVE FIELD name.*.inner[0] ON what);
	let expected = parse("REMOVE FIELD name.*.inner[0] ON what").unwrap();
	assert_eq!(query, expected);

	let query = sql!(REMOVE FIELD name[*].inner[0] ON what);
	let expected = parse("REMOVE FIELD name[*].inner[0] ON what").unwrap();
	assert_eq!(query, expected);
}

#[test]
fn remove_index() {
	let query = sql!(REMOVE INDEX name ON what);
	let expected = parse("REMOVE INDEX name ON what").unwrap();
	assert_eq!(query, expected);
}
