use crate::sql::{DBObject, TypeWriter};
use crate::type_writers::Postgresql;

pub type BxTypeWriter = Box<dyn TypeWriter>;

/// DML processor and SQL generator
///
/// Collects DBObject's and creates SQL statements using the supplied
///  TypeWriter or Postgresql if none is provided
pub struct Processor {
    out: Vec<String>,
    type_writer:BxTypeWriter,
}

impl Processor {
    /// Create a new Processor optionally specifying a TypeWriter to use
    pub fn new(type_writer:Option<BxTypeWriter>) -> Self {
        let type_writer = if let Some(tr) = type_writer {
            tr
        } else {
            Box::new(Postgresql{})
        };
        Processor {
            out: Vec::new(),
            type_writer,
        }
    }
    /// Add a DB object to serialize to SQL
    pub fn add(&mut self, object:&dyn DBObject) -> &Self {
        self.out.push(object.to_sql(self.type_writer.as_ref()));
        self
    }
    /// Get the list of serialized SQL statements
    pub fn statements(&self) -> &Vec<String> {
        &self.out
    }
    /// Get a String with all of the SQL statments
    pub fn to_string(&self) -> String {
        self.out.join("\n")
    }
}
