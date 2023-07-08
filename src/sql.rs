use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::option::*;

pub trait PostgresObject {
    fn to_sql(&self) -> String;
}
pub trait MySQLObject {
    fn to_sql(&self) -> String;
}

pub type DBObject = dyn PostgresObject;
// pub type DBObject = dyn MySQLObject;

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_type() -> FieldType {
    FieldType::Txt
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum FieldType {
    #[serde(rename = "int")]
    Int,
    #[serde(rename = "bigint")]
    BigInt,
    #[serde(rename = "text")]
    Txt,
    #[serde(rename = "double")]
    Dbl,
    #[serde(rename = "bool")]
    Bool,
    #[serde(rename = "auto_increment")]
    AutoInc,
}
impl ToString for FieldType {
    fn to_string(&self) -> String {
        match self {
            FieldType::Int => "int".to_owned(),
            FieldType::BigInt => "bigint".to_owned(),
            FieldType::Txt => "text".to_owned(),
            FieldType::Bool => "bool".to_owned(),
            FieldType::Dbl => "double precision".to_owned(),
            FieldType::AutoInc => "serial".to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FieldValue {
    Int(i32),
    BigInt(i64),
    Txt(String),
    Dbl(f64),
    Bool(bool),
    Empty,
}

// #[derive(Default)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FieldAttributes {
    #[serde(rename = "type", default="default_type")]
    pub dtype: FieldType,
    #[serde(default="default_false")]
    pub unique: bool,
    #[serde(default="default_true")]
    pub empty: bool,
    #[serde(default="default_false")]
    pub roster: bool,
    pub defval: Option<String>,
    #[serde(default="default_false")]
    pub primary_key: bool,
    #[serde(default="default_false")]
    pub index: bool,
    #[serde(default="default_false")]
    pub only_db: bool,
    // name when searching in InterData
    pub meta_name: Option<String>,
}
impl FieldAttributes {
    fn new_default(dt:FieldType) -> Self {
        FieldAttributes{
            dtype: dt,
            unique: false,
            empty: true,
            roster: false,
            defval: None,
            primary_key: false,
            index: false,
            only_db: false,
            meta_name: None,
        }
    }
    pub fn new(dt:FieldType) -> Self {
        FieldAttributes::new_default(dt)
    }
    pub fn new_nn(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_default(dt);
        me.empty = false;
        me
    }
    pub fn new_nn_idx(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_nn(dt);
        me.index = true;
        me
    }
    pub fn new_nn_def(dt:FieldType, defval:&str) -> Self {
        let mut me = FieldAttributes::new_nn(dt);
        me.defval = Some(defval.to_string());
        me
    }
    pub fn new_pk(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_nn(dt);
        me.primary_key = true;
        me
    }
    pub fn new_uk(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_default(dt);
        me.unique = true;
        me
    }
    pub fn new_uk_pk(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_uk(dt);
        me.primary_key = true;
        me
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub attributes: FieldAttributes,
}

impl Field {
    pub fn new(name:&str, attrs:&FieldAttributes) -> Self {
        Field {
            name:name.to_string(),
            attributes:attrs.clone(),
        }
    }
    pub fn new_only_db(name:&str, attrs:&FieldAttributes) -> Self {
        let mut me = Field::new(name, attrs);
        me.attributes.only_db = true;
        me
    }
}
impl PostgresObject for Field {
    fn to_sql(&self) -> String {
        let mut s = match self.name.as_str() {
            "role"=>format!("\"role\""),
            _=>self.name.to_owned(),
        };            
        let att = &self.attributes;
        s += format!(" {} ", att.dtype.to_string()).as_str();
        s += if att.empty {
            "NULL"
        } else {
            "NOT NULL"
        };
        if let Some(def) = &att.defval {
            s += format!(" DEFAULT {}", match att.dtype {
                FieldType::Txt => format!("\"{}\"", def),
                _=>def.to_owned()
            }).as_str()
        }
        s
    }
}

pub type Fields = Vec<Field>;
type FieldNames = Vec<String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum GrantType {
    Select,
    Insert,
    Update,
    Delete,
    Truncate,
    References,
    Trigger,
    All,
    Usage,
}
impl ToString for GrantType {
    fn to_string(&self) -> String {
        match self {
            GrantType::Select=>"SELECT".to_owned(),
            GrantType::Insert=>"INSERT".to_owned(),
            GrantType::Update=>"UPDATE".to_owned(),
            GrantType::Delete=>"DELETE".to_owned(),
            GrantType::Truncate=>"TRUNCATE".to_owned(),
            GrantType::References=>"REFERENCES".to_owned(),
            GrantType::Trigger=>"TRIGGER".to_owned(),
            GrantType::All=>"ALL".to_owned(),
            GrantType::Usage=>"USAGE".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grant {
    pub permission: GrantType,
    pub to: String,
    pub on: ObjectPath,
}
impl Grant {
    pub fn new(perm:GrantType, to:&str, on:&ObjectPath) -> Self {
        Grant { permission: perm.to_owned(), to: to.to_string(), on: on.to_owned() }
    }
}
impl PostgresObject for Grant {
    fn to_sql(&self) -> String {
        format!("GRANT {} ON {} {} TO {};", self.permission.to_string(), self.on.otype.to_string(), self.on.full_name(), self.to)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owner {
    pub to: String,
    pub of: ObjectPath,
}
impl Owner {
    pub fn new(to:&str, of:&ObjectPath) -> Self {
        Owner { to: to.to_string(), of: of.to_owned() }
    }
}
impl PostgresObject for Owner {
    fn to_sql(&self) -> String {
        format!("ALTER {} {} OWNER TO {};", self.of.otype.to_string(), self.of.full_name(), self.to)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub table: ObjectPath,
    pub fields: FieldNames,
}


type Indexes = Vec<Index>;


#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueKey {
    name: String,
    fields: FieldNames, 
}
impl PostgresObject for UniqueKey {
    fn to_sql(&self) -> String {
        format!("CONSTRAINT {}_uk UNIQUE ({})", self.name, self.fields.join(","))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryKey {
    name: String,
    fields: FieldNames, 
}
impl PostgresObject for PrimaryKey {
    fn to_sql(&self) -> String {
        format!("CONSTRAINT {}_pk PRIMARY KEY ({})", self.name, self.fields.join(","))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FKOn {
    Restrict,
    Cascade,
}
impl ToString for FKOn {
    fn to_string(&self) -> String {
        match self {
            FKOn::Cascade=>"CASCADE".to_owned(),
            FKOn::Restrict=>"RESTRICT".to_owned(),
        }
    }
}

fn default_on_clause() -> FKOn {
    FKOn::Restrict
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ForeignKey {
    pub table: ObjectPath,
    pub fields: FieldNames,
    pub ref_table: ObjectPath,
    pub ref_fields: FieldNames,
    #[serde(default="default_on_clause")]
    pub on_delete: FKOn,
    #[serde(default="default_on_clause")]
    pub on_update: FKOn,
}
impl PostgresObject for ForeignKey {
    fn to_sql(&self) -> String {
        format!("ALTER TABLE {}\n  ADD CONSTRAINT {}_{}_{}_fk\n  FOREIGN KEY ({})\n  REFERENCES {} ({})\n  ON DELETE {} ON UPDATE {};",
                self.table.full_name(),
                self.table.name, self.ref_table.name, self.fields.join("_"),
                self.fields.join(","),
                self.ref_table.full_name(),
                self.ref_fields.join(","),
                self.on_delete.to_string(),
                self.on_update.to_string()
        )
    }
}

impl PostgresObject for Index {
    fn to_sql(&self) -> String {
        format!("CREATE INDEX {}_{}_idx ON {} USING btree ({});",
                self.table.name, self.fields.join("_"), self.table.full_name(),
                self.fields.join(","))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ObjectType {
    Table,
    Sequece,
    Schema,
}
impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            ObjectType::Table=>"TABLE".to_owned(),
            ObjectType::Sequece=>"SEQUENCE".to_owned(),
            ObjectType::Schema=>"SCHEMA".to_owned(),
        }
    }
}
impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Table
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectPath {
    pub schema: Option<String>,
    pub name: String,
    #[serde(default)]
    pub otype: ObjectType,
}
impl ObjectPath {
    pub fn new_table(schema:&str, name:&str) -> Self {
        ObjectPath { schema: Some(schema.to_string()), name: name.to_string(), otype:ObjectType::Table }
    }
    pub fn new_schema(name:&str) -> Self {
        ObjectPath { schema: None, name: name.to_string(), otype:ObjectType::Schema }
    }
    pub fn new_sequence(schema:&str, name:&str) -> Self {
        ObjectPath { schema: Some(schema.to_string()), name: name.to_string(), otype:ObjectType::Sequece }
    }
    pub fn full_name(&self) -> String {
        if let Some(schema) = &self.schema {
            format!("{}.{}", schema, self.name)
        } else {
            self.name.to_owned()
        }
    }
}

pub struct Table {
    pub path: ObjectPath,
    fields: Fields,
    constraints: Option<Vec<Box<DBObject>>>,
}
impl Table {
    pub fn new(path:&ObjectPath, fields:Fields/*, consts: Option<Vec<Box<DBObject>>>*/) -> Self {
        let mut me = Table { path: path.to_owned(), fields, constraints:None };
        // lets check for duplicates
        {
            let mut unicos = HashSet::new();
            let mut dups = Vec::new();
            for f in me.fields.iter() {
                if ! unicos.insert(&f.name) {
                    dups.push(&f.name)
                }
            }
            if ! dups.is_empty() {                
                panic!("{} has duplicated fields: {dups:?}", me.path.full_name())
            }
        }
        let mut cts:Vec<Box<DBObject>> = Vec::new();
        let mut uks:Vec<String> = Vec::new();
        let mut pks:Vec<String> = Vec::new();
        for f in me.fields.iter() {
            if f.attributes.primary_key {
                pks.push(f.name.to_owned())
            }
            if f.attributes.unique {
                uks.push(f.name.to_owned())
            }
        }
        if ! pks.is_empty() {
            cts.push(Box::new(PrimaryKey{ name: format!("{}_{}", me.path.name, pks.join("_")), fields:pks }))
        }
        if ! uks.is_empty() {
            cts.push(Box::new(UniqueKey{ name: format!("{}_{}", me.path.name, uks.join("_")), fields:uks}))
        }
        if ! cts.is_empty() {
            me.constraints = Some(cts)
        }
        me
    }
    pub fn indexes(&self) -> Option<Indexes> {
        let mut idxs:Vec<String> = Vec::new();
        for f in self.fields.iter() {
            if f.attributes.index {
                idxs.push(f.name.clone())
            }
        }
        if ! idxs.is_empty() {
            Some(vec![Index{
                table:self.path.to_owned(),
                fields:idxs
            }])
        } else {
            None
        }
    }
}
impl PostgresObject for Table {
    fn to_sql(&self) -> String {
        let cols : Vec<String> = self.fields.iter().map(|f| f.to_sql().to_owned()).collect();
        let refs = if let Some(constraints) = &self.constraints {
            let v: Vec<String> = constraints.iter().map(|f| f.to_sql().to_owned()).collect();
            v
        } else {
            Vec::new()
        };
        let mut t=format!("CREATE TABLE {} (\n  {}", self.path.full_name(), cols.join(",\n  "));
        if ! refs.is_empty() {
            t += format!(",\n  {}", refs.join(",\n  ")).as_str()
        }
        t += "\n);";
        t
    }
}

pub struct Schema {
    pub name: String,
    pub owner: String,
}
impl Schema {
    pub fn new(name:&str, owner:&str) -> Self {
        Schema{ name: name.to_string(), owner: owner.to_string(), }
    }
}
impl PostgresObject for Schema {
    fn to_sql(&self) -> String {
        format!("CREATE SCHEMA {} AUTHORIZATION {};", self.name, self.owner)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::*;
    use crate::util::write_yaml_to_file;

    #[test]
    fn test_fields() {
        let fields = read_asg_fields("fixtures/test-fields.yaml").expect("to open test-fields.yaml");
        // println!("fields: {fields:#?}");
        assert_eq!(6, fields.basic.len());
        let t = fields.basic.get("id").expect("to get id");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Txt, unique:false, empty: false, roster: false, defval: None,
            primary_key:true, index: false, only_db:false, meta_name: None,
        });
        let t = fields.basic.get("pk").expect("to get pk");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Txt, unique:false, empty: true, roster: false, defval: None,
            primary_key:false, index: false, only_db:false, meta_name: None,
        });
        let t = fields.basic.get("ws").expect("to get ws");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Txt, unique: false, empty: true, roster: false, defval: None,
            primary_key:true, index: false, only_db:false, meta_name: None,
        });
        let t = fields.basic.get("gallo").expect("to get gallo");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Int, unique: false, empty: true, roster: true, defval: None,
            primary_key:false, index: true, only_db:false, meta_name: None,
        });
        let t = fields.basic.get("vivo").expect("to get vivo");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Bool, unique: false, empty: true, roster: false, defval: Some("true".into()),
            primary_key:false, index: true, only_db:false, meta_name: None,
        });
        let t = fields.basic.get("tel").expect("to get tel");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Txt, unique: false, empty: true, roster: false, defval: None,
            primary_key:false, index: false, only_db:false, meta_name: Some("TEL".into()),
        });

        assert!(fields.sensitization.is_some());
        let sens = fields.sensitization.expect("sensitization");
        assert_eq!(3, sens.len());
        let t = sens.get("hog_sens").expect("to get hog_sens");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Int, unique: false, empty: true, roster: true, defval: None,
            primary_key:false, index: false, only_db:false, meta_name: None,
        });
        let t = sens.get("pct_life").expect("to get pct_life");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::Dbl, unique: false, empty: true, roster: false, defval: None,
            primary_key:false, index: false, only_db:false, meta_name: None,
        });
        let t = sens.get("id").expect("to get id");
        assert_eq!(t, &FieldAttributes{
            dtype:FieldType::AutoInc, unique: false, empty: true, roster: false, defval: None,
            primary_key:false, index: false, only_db:false, meta_name: None,
        });

    }
    #[test]
    fn test_tables() {
        use std::fs;
        
        let afields = read_asg_fields("fixtures/test-fields.yaml").expect("to open test-fields.yaml");
        // println!("afields: {afields:#?}");
        let fields: Fields = afields.basic.iter().map(|(k,v)| Field::new( k, v)).collect();
        // println!("fields: {fields:#?}");
        let tbl = Table::new(&ObjectPath::new_table("demo", "prueba"), fields);
        // println!("\n{}", tbl.to_sql());
        let ttf="fixtures/test-table.sql";
        assert_eq!(tbl.to_sql(), fs::read_to_string(ttf).expect(ttf));
        if let Some(indexes) = tbl.indexes() {
            let tif="fixtures/test-table-idx.sql";
            assert_eq!(indexes[0].to_sql(), fs::read_to_string(tif).expect(tif));
        }
        
        let fk = ForeignKey{
            table:tbl.path.to_owned(),
            // table:ObjectPath::new("demo", "prueba"),
            fields:vec!["ws_id".to_owned(), "user_id".to_owned()],
            ref_table:ObjectPath::new_table("demo", "cache"),
            ref_fields:vec!["ws".to_owned(), "user".to_owned()],
            on_delete:FKOn::Restrict, on_update:FKOn::Restrict,
        };
        write_yaml_to_file("local-prueba.yaml", &fk);
        // println!("{}", fk.to_string());
        let tfk="fixtures/test-table-fks.sql";
        assert_eq!(fk.to_sql(), fs::read_to_string(tfk).expect(tfk));
        

    }

}
