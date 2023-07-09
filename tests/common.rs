
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::error::Error;
use dml_tools::sql::*;
use dml_tools::util::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestTables {
    pub basic: DynFields,
    pub sensitization: Option<DynFields>,
    pub foreign_keys: Option<ForeignKeys>,
}

#[allow(dead_code)]
pub fn read_test_tables<P: AsRef<Path>>(path: P) -> Result< TestTables, Box<dyn Error>> {
    Ok(read_yaml_from_file(path)?)
}

#[allow(dead_code)]
pub fn test_table_with_writer_to_string(writer:&dyn TypeWriter) -> String {
    let fields = vec![
        Field::new("id", &FieldAttributes::new_nn(FieldType::AutoInc)),
        Field::new("age", &FieldAttributes::new(FieldType::Int)),
        Field::new("year_of_birth", &FieldAttributes::new(FieldType::BigInt)),
        Field::new("name", &FieldAttributes::new(FieldType::Txt)),
        Field::new("weight", &FieldAttributes::new(FieldType::Dbl)),
        Field::new("is_married", &FieldAttributes::new(FieldType::Bool)),
    ];
    let table = Table::new(&ObjectPath::new_table("myschema", "mytable"), fields);
    table.to_sql(writer)
}
