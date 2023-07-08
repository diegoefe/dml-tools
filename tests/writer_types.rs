use dml_tools::util::read_file_into_string;
use dml_tools::writers::*;

mod common;
use common::*;

#[test]
fn test_writer_postgresql() {
    let writer = Box::new(PostgresqlTypeWriter{});
    assert_eq!(test_table_with_writer_to_string(writer.as_ref()), read_file_into_string("tests/fixtures/table-postgresql.sql"));
}

#[test]
fn test_writer_mysql() {
    let writer = Box::new(MysqlTypeWriter{});
    assert_eq!(test_table_with_writer_to_string(writer.as_ref()), read_file_into_string("tests/fixtures/table-mysql.sql"));
}


#[test]
fn test_writer_sqlite() {
    let writer = Box::new(SqliteTypeWriter{});
    assert_eq!(test_table_with_writer_to_string(writer.as_ref()), read_file_into_string("tests/fixtures/table-sqlite.sql"));
}
