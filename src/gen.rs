use crate::sql::{DBObject, TypeWriter};
use crate::type_writers::Postgresql;
use crate::util::write_yaml_to_file;
use std::error::Error;

pub type BxTypeWriter = Box<dyn TypeWriter>;

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
    pub fn new_with_objects(objects:&'a Vec<Box<dyn DBObject>>, type_writer:Option<BxTypeWriter>) -> Self {
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
    pub fn add_all(&mut self, object:&'a dyn DBObject) -> &Self {
        self.objs.push(Box::new(object));
        self
    }
    /// Get the list of serialized SQL sql_statements
    pub fn sql_statements(&self) -> Vec<String> {
        let mut out = Vec::new();
        for obj in &self.objs {
            out.push(obj.to_sql(self.type_writer.as_ref()))
        }
        out
    }
    /// Get a String with all of the SQL statments
    pub fn to_string(&self) -> String {
        self.sql_statements().join("\n")
    }
    /// Write objects to a YAML file
    pub fn write_to_file(&self, file_name:&str) -> Result<(), Box<dyn Error>> {
        Ok(write_yaml_to_file(file_name, &self.objs)?)
    }
}
