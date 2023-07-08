use serde::{Deserialize, Serialize};
// use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;
use crate::util;
use std::path::Path;
use std::error::Error;

use crate::sql::{FieldAttributes,ForeignKey,ObjectPath};


// type DynFields = HashMap<String, FieldAttributes>;
type DynFields = LinkedHashMap<String, FieldAttributes>;
type ForeingKeys = Vec<ForeignKey>;

#[derive(Serialize, Deserialize, Debug)]
pub struct AsgFields {
    pub basic: DynFields,
    pub sensitization: Option<DynFields>,
    pub fks: Option<ForeingKeys>,
}

pub fn read_asg_fields<P: AsRef<Path>>(path: P) -> Result< AsgFields, Box<dyn Error>> {
    Ok(util::read_yaml_from_file(path)?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AsgRoles {
    pub rw: String,
    pub ro: String,
    pub upd: String,
}
impl Default for AsgRoles {
    fn default() -> Self {
        AsgRoles {
            rw: "dnecv_rw".into(),
            ro: "dnecv_ro".into(),
            upd: "dnecv_upd".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AsgTables {
    main: String,
    cache: String,
    users: String,
}
impl Default for AsgTables {
    fn default() -> Self {
        AsgTables {
            main: "asignaciones".into(),
            cache: "asignaciones_cache".into(),
            users: "users".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssignSpec {
    pub schema: String,
    #[serde(default)]
    pub tables: AsgTables,
    #[serde(default)]
    pub roles: AsgRoles,
    pub fields_file: String, // points to AsgFields aware file
}
impl AssignSpec {
    pub fn table_main(&self) -> String {
        format!("{}.{}", self.schema, self.tables.main)
    }
    pub fn table_cache(&self) -> String {
        format!("{}.{}", self.schema, self.tables.cache)
    }
    pub fn table_users(&self) -> String {
        format!("{}.{}", self.schema, self.tables.users)
    }
    pub fn path_table_main(&self) -> ObjectPath {
        ObjectPath::new_table(&self.schema, &self.tables.main)
    }
    pub fn path_table_cache(&self) -> ObjectPath {
        ObjectPath::new_table(&self.schema, &self.tables.cache)
    }
    pub fn path_table_users(&self) -> ObjectPath {
        ObjectPath::new_table(&self.schema, &self.tables.users)
    }
}

