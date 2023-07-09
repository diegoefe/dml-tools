use crate::sql::{DBObject, TypeWriter};
use crate::type_writers::Postgresql;

//#[cfg(windows)]
//const LINE_ENDING: &'static str = "\r\n";
//#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

pub type BxTypeWriter = Box<dyn TypeWriter>;

pub struct Generator {
    out: Vec<String>,
    type_writer:BxTypeWriter,
}

impl Generator {
    pub fn new(type_writer:Option<BxTypeWriter>) -> Self {
        let type_writer = if let Some(tr) = type_writer {
            tr
        } else {
            Box::new(Postgresql{})
        };
        Generator {
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
        self.out.join(LINE_ENDING)
    }
}
