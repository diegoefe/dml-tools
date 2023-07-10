//! A library for DML generation from code or YAML files

mod gen;
pub use gen::Processor;

/// TypeWriters
///
/// Defines TypeWriters for Postgresql (default), Mysql and Sqlite
pub mod type_writers;

/// SQL module
///
/// Defines many type of database objects to generate SQL sql_statements from
pub mod sql;

/// Util module
///
/// Contains easy to use functions to read and write to YAML files
pub mod util;