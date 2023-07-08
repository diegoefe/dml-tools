use crate::sql::{TypeWriter, FieldType};

#[derive(Debug)]
pub struct PostgresqlTypeWriter {}
impl TypeWriter for PostgresqlTypeWriter {
    fn type_to_sql(&self, field_type:&FieldType) -> String {
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

#[derive(Debug)]
pub struct MysqlTypeWriter {}
impl TypeWriter for MysqlTypeWriter {
    fn type_to_sql(&self, field_type:&FieldType) -> String {
        match field_type {
            FieldType::Int => "int".to_owned(),
            FieldType::BigInt => "bigint".to_owned(),
            FieldType::Txt => "varchar".to_owned(),
            FieldType::Bool => "bool".to_owned(),
            FieldType::Dbl => "double".to_owned(),
            FieldType::AutoInc => "int autoincrement".to_owned(),
        }
    }
}