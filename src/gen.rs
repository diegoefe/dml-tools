use crate::sql::{DBObject, TypeWriter};
use crate::type_writers::Postgresql;

pub type BxTypeWriter = Box<dyn TypeWriter>;

/// DML Processor
///
/// Collects DBObject's and creates SQL statements using the supplied
///  TypeWriter or Postgresql if none is provided
pub struct Processor {
    out: Vec<String>,
    type_writer:BxTypeWriter,
}

impl Processor {
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
    pub fn add(&mut self, object:&dyn DBObject) -> &Self {
        self.out.push(object.to_sql(self.type_writer.as_ref()));
        self
    }
    pub fn statements(&self) -> &Vec<String> {
        &self.out
    }
    pub fn to_string(&self) -> String {
        self.out.join("\n")
    }
}
