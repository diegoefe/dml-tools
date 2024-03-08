use crate::sql::{DBObject, TypeWriter};
use crate::type_writers::Postgresql;
use crate::util::*;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

type BxTypeWriter = Box<dyn TypeWriter>;
type Objects = Vec<Box<dyn DBObject>>;

/// DML processor and SQL generator
///
/// Collects DBObject's and creates SQL sql_statements using the supplied
///  TypeWriter or Postgresql if none is provided
pub struct Processor<'a> {
    objs: Vec<Box<&'a dyn DBObject>>,
    type_writer:BxTypeWriter,
}

impl <'a> Processor<'a> {
    /// Create a new Processor optionally specifying a TypeWriter to use
    pub fn new(type_writer:Option<BxTypeWriter>) -> Self {
        let type_writer = if let Some(tr) = type_writer {
            tr
        } else {
            Box::new(Postgresql{})
        };
        Processor {
            objs: Vec::new(),
            type_writer,
        }
    }
    /// Create a new Processor optionally specifying a TypeWriter to use
    pub fn new_with_objects(objects:&'a Objects, type_writer:Option<BxTypeWriter>) -> Self {
        let mut me = Processor::new(type_writer);
        for obj in objects.iter() {
            me.objs.push(Box::new(obj.as_ref()))
        }
        me
    }
    /// Add a DB object
    pub fn add(&mut self, object:&'a dyn DBObject) -> &Self {
        self.objs.push(Box::new(object));
        self
    }
    /// Get the list of serialized SQL sql_statements
    pub fn sql_statements(&self) -> Vec<String> {
        let mut out = Vec::new();
        let mut delayed = Vec::new();
        for obj in &self.objs {
            if ! obj.is_top_level() {
                delayed.push(obj)
            } else {
                let sql;
                if delayed.is_empty() {
                    sql = obj.to_sql(self.type_writer.as_ref())
                } else {
                    sql = obj.top_level_to_sql(self.type_writer.as_ref(), &delayed);
                    delayed.clear()
                };
                if ! sql.is_empty() {
                    out.push(sql);
                }
            }
        }
        out
    }
    /// Get a String with all of the SQL statments
    pub fn join_sql_statements(&self) -> String {
        self.sql_statements().join("\n")
    }
    /// Write objects to a YAML file
    pub fn serialize_to_yaml_file(&self, file_name:&str) -> Result<(), Box<dyn Error>> {
        Ok(write_yaml_to_file(file_name, &self.objs)?)
    }
    /// Write generated SQL to file
    pub fn write_to_sql_file(&self, file_name:&str) -> Result<(), Box<dyn Error>> {
        let sqls = self.join_sql_statements();
        let mut fh = File::create(file_name)?;
        Ok(fh.write_all(&sqls.as_bytes())?)
    }
    /// Get number of objects present
    pub fn objects(&self) -> &Vec<Box<&'a dyn DBObject>> {
        &self.objs
    }
}


/// DML yaml loader
///
/// Loads DBObject's from a .yaml file and permits accesing them
/// Used for giving scope to Processor
pub struct Loader {
    objs: Objects,
}

impl Loader {
    /// Create Loader from YAML in a String
    pub fn new(data:&str) -> Result<Self, Box<dyn Error>> {
        Ok(Loader {
            objs: read_yaml_from_string(data).expect(format!("To load objects from string '{data}'").as_str())
        })
    }
    /// Create Loader reading from a YAML file
    pub fn new_from_file(file_name:&str) -> Result<Self, Box<dyn Error>> {
        Ok(Loader {
            objs: read_yaml_from_file(file_name).expect(format!("To load objects from '{file_name}'").as_str())
        })
    }
    pub fn objects(&self) -> &Objects {
        &self.objs
    }
}
