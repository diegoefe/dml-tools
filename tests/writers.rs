use dml_tools::util::read_file_into_string;
use dml_tools::type_writers::*;

mod common;
use common::*;

#[test]
fn test_writer_postgresql() {
    let writer = Box::new(Postgresql{});
    assert_eq!(test_table_with_writer_to_string(writer.as_ref()), read_file_into_string("tests/fixtures/table-postgresql.sql"));
}

#[test]
fn test_writer_mysql() {
    let writer = Box::new(Mysql{});
    assert_eq!(test_table_with_writer_to_string(writer.as_ref()), read_file_into_string("tests/fixtures/table-mysql.sql"));
}


#[test]
fn test_writer_sqlite() {
    let writer = Box::new(Sqlite{});
    assert_eq!(test_table_with_writer_to_string(writer.as_ref()), read_file_into_string("tests/fixtures/table-sqlite.sql"));
}
