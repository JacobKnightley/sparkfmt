use sparkfmt_core::format_sql;

// DDL Tests

#[test]
fn test_create_table_simple() {
    let input = "create table users (id int, name string, email string)";
    let expected = "CREATE TABLE users (
     id INT
    ,name STRING
    ,email STRING
)";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_drop_table() {
    let input = "drop table users";
    let expected = "DROP TABLE users";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_drop_table_if_exists() {
    let input = "drop table if exists users";
    let expected = "DROP TABLE IF EXISTS users";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_describe_table() {
    let input = "describe users";
    let expected = "DESCRIBE users";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_describe_extended() {
    let input = "describe extended users";
    let expected = "DESCRIBE EXTENDED users";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_show_tables() {
    let input = "show tables";
    let expected = "SHOW TABLES";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_show_tables_in_database() {
    let input = "show tables in mydb";
    let expected = "SHOW TABLES IN mydb";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

// DML Tests

#[test]
fn test_insert_into_select() {
    let input = "insert into target select id, name from source";
    let expected = "INSERT INTO target
SELECT
     id
    ,name
FROM source";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_insert_into_select_with_where() {
    let input = "insert into target select id, name from source where active = true";
    let expected = "INSERT INTO target
SELECT
     id
    ,name
FROM source
WHERE active=true";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_delete_from() {
    let input = "delete from users where id = 123";
    let expected = "DELETE FROM users
WHERE id=123";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_delete_from_multiple_conditions() {
    let input = "delete from users where created < '2020-01-01' and status = 'inactive'";
    let expected = "DELETE FROM users
WHERE
    created<'2020-01-01'
    AND status='inactive'";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

// Session Tests

#[test]
fn test_set_config() {
    let input = "set spark.sql.shuffle.partitions = 200";
    let expected = "SET spark.sql.shuffle.partitions=200";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_use_database() {
    let input = "use my_database";
    let expected = "USE my_database";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_use_database_with_keyword() {
    let input = "use database analytics";
    let expected = "USE analytics";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

// Combined Tests with Comments

#[test]
fn test_create_table_with_leading_comment() {
    let input = "-- User table\ncreate table users (id int, name string)";
    let expected = "-- User table\nCREATE TABLE users (
     id INT
    ,name STRING
)";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_insert_with_leading_comment() {
    let input = "-- Load data\ninsert into target select * from source";
    let expected = "-- Load data\nINSERT INTO target
SELECT
     *
FROM source";
    let result = format_sql(input).unwrap();
    assert_eq!(result, expected);
}
