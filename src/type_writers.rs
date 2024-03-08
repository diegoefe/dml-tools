use crate::sql::{TypeWriter, FieldType, ObjectPath};

/// PostgreSQL type serializator
#[derive(Debug)]
pub struct Postgresql {}
impl TypeWriter for Postgresql {
    fn id(&self) -> &str { "pgsql" }
    fn field_type(&self, field_type:&FieldType) -> String {
        match field_type {
            FieldType::Int => "int".to_owned(),
            FieldType::BigInt => "bigint".to_owned(),
            FieldType::Txt => "text".to_owned(),
            FieldType::Bool => "bool".to_owned(),
            FieldType::Dbl => "double precision".to_owned(),
            FieldType::AutoInc => "serial".to_owned(),
        }
    }
    fn supports_sequences(&self) -> bool { true }
}

/// MySQL type serializator
#[derive(Debug)]
pub struct Mysql {}
impl TypeWriter for Mysql {
    fn id(&self) -> &str { "mysql" }
    fn field_type(&self, field_type:&FieldType) -> String {
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
    fn id(&self) -> &str { "sqlite" }
    fn field_type(&self, field_type:&FieldType) -> String {
        match field_type {
            FieldType::Int => "integer".to_owned(),
            FieldType::BigInt => "integer".to_owned(),
            FieldType::Txt => "text".to_owned(),
            FieldType::Bool => "integer".to_owned(),
            FieldType::Dbl => "real".to_owned(),
            FieldType::AutoInc => "integer primary key autoincrement".to_owned(),
        }
    }
    fn schema(&self, op:&ObjectPath) -> String {
        op.name.to_owned()
    }
    fn index_type(&self) -> String { "".to_string() }
    fn supports_schemas(&self) -> bool { false }
    fn supports_permissions(&self) -> bool { false }
    fn supports_auto_increment(&self) -> bool { false }
}