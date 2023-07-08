
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::error::Error;
use dml_tools::sql::*;
use dml_tools::util::read_yaml_from_file;

#[derive(Serialize, Deserialize, Debug)]
pub struct TestTables {
    pub basic: DynFields,
    pub sensitization: Option<DynFields>,
    pub foreign_keys: Option<ForeingKeys>,
}

pub fn read_test_tables<P: AsRef<Path>>(path: P) -> Result< TestTables, Box<dyn Error>> {
    Ok(read_yaml_from_file(path)?)
}
