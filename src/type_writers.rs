use crate::sql::{TypeWriter, FieldType};

/// PostgreSQL type serializator
#[derive(Debug)]
pub struct Postgresql {}
impl TypeWriter for Postgresql {
    fn field_to_sql(&self, field_type:&FieldType) -> String {
        match field_type {
            FieldType::Int => "int".to_owned(),
            FieldType::BigInt => "bigint".to_owned(),
            FieldType::Txt => "text".to_owned(),
            FieldType::Bool => "bool".to_owned(),
            FieldType::Dbl => "double precision".to_owned(),
            FieldType::AutoInc => "serial".to_owned(),
        }
    }
}

/// MySQL type serializator
#[derive(Debug)]
pub struct Mysql {}
impl TypeWriter for Mysql {
    fn field_to_sql(&self, field_type:&FieldType) -> String {
        match field_type {
            FieldType::Int => "int".to_owned(),
            FieldType::BigInt => "bigint".to_owned(),
            FieldType::Txt => "text".to_owned(),
            FieldType::Bool => "bit".to_owned(),
            FieldType::Dbl => "double".to_owned(),
            FieldType::AutoInc => "integer auto_increment".to_owned(),
        }
    }
}

/// SQLite type serializator
#[derive(Debug)]
pub struct Sqlite {}
impl TypeWriter for Sqlite {
    fn field_to_sql(&self, field_type:&FieldType) -> String {
        match field_type {
            FieldType::Int => "integer".to_owned(),
            FieldType::BigInt => "integer".to_owned(),
            FieldType::Txt => "text".to_owned(),
            FieldType::Bool => "integer".to_owned(),
            FieldType::Dbl => "real".to_owned(),
            FieldType::AutoInc => "autoincrement".to_owned(),
        }
    }
}