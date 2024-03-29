use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt::Debug;
use std::option::*;
use linked_hash_map::LinkedHashMap;

// use crate::Processor;

/// Trait for a type that can convert a FieldType to String
pub trait TypeWriter {
    fn id(&self) -> &str;
    fn field_type(&self, field_type:&FieldType) -> String;
    fn schema(&self, op:&ObjectPath) -> String {
        if let Some(schema) = &op.schema {
            format!("{}.{}", schema, op.name)
        } else {
            op.name.to_owned()
        }
    }
    fn index_type(&self) -> String { " USING btree".to_string() }
    fn supports_schemas(&self) -> bool { true }
    fn supports_permissions(&self) -> bool { true }
    fn supports_auto_increment(&self) -> bool { true }
    fn supports_sequences(&self) -> bool { false }
}

/// Trait for serializing a database object to as String
// #[typetag::serde(tag = "type")]
#[typetag::serde(tag = "tag")]
pub trait DBObject : Debug {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String;
    fn is_top_level(&self) -> bool { false }
    fn top_level_to_sql(&self, _type_writer:&dyn TypeWriter, _delayed: &Vec<&Box<& dyn DBObject>>) -> String {
        if self.is_top_level() {
            panic!("should not run a non-reimplemented top_level_to_sql()!")
        }
        "".to_owned()
    }
}

fn default_false() -> bool {
    false
}

fn default_true() -> bool {
    true
}

fn default_type() -> FieldType {
    FieldType::Txt
}

fn is_default_dtype(t:&FieldType) -> bool {
    *t == default_type()
}

fn is_default_false(b:&bool) -> bool {
    *b == false
}
fn is_default_true(b:&bool) -> bool {
    *b == true
}
fn is_none(opt:&Option<String>) -> bool {
    opt.is_none()
}
/// Types of table fields
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
#[typetag::serde]
impl DBObject for FieldType {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        type_writer.field_type(&self)
    }
}

/// Attributes for fields
// #[derive(Default)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct FieldAttributes {
    /// Type of field
    #[serde(rename = "type", default="default_type")]
    #[serde(skip_serializing_if = "is_default_dtype")]
    pub dtype: FieldType,
    /// Is it a UNIQUE field?
    #[serde(default="default_false")]
    #[serde(skip_serializing_if = "is_default_false")]
    pub unique: bool,
    /// Can be NULL?
    #[serde(default="default_true")]
    #[serde(skip_serializing_if = "is_default_true")]
    pub empty: bool,
    /// Is it a roster?
    #[serde(default="default_false")]
    #[serde(skip_serializing_if = "is_default_false")]
    pub roster: bool,
    /// Optional default value for thes field
    #[serde(skip_serializing_if = "is_none")]
    pub defval: Option<String>,
    #[serde(default="default_false")]
    #[serde(skip_serializing_if = "is_default_false")]
    /// Is it PRIMARY KEY
    pub primary_key: bool,
    #[serde(default="default_false")]
    #[serde(skip_serializing_if = "is_default_false")]
    /// Is it INDEXed?
    pub index: bool,
    /// Should not be used for De/Serialization?
    #[serde(default="default_false")]
    #[serde(skip_serializing_if = "is_default_false")]
    pub only_db: bool,
    /// Optional name for this field
    // name when searching in InterData
    #[serde(skip_serializing_if = "is_none")]
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
    /// Initialize default FieldAttributes for this FieldType
    pub fn new(dt:FieldType) -> Self {
        FieldAttributes::new_default(dt)
    }
    /// Initialize NOT NULL FieldAttributes for this FieldType
    pub fn new_nn(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_default(dt);
        me.empty = false;
        me
    }
    /// Initialize NOT NULL + INDEX FieldAttributes for this FieldType
    pub fn new_nn_idx(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_nn(dt);
        me.index = true;
        me
    }
    /// Initialize NOT NULL + default value FieldAttributes for this FieldType
    pub fn new_nn_def(dt:FieldType, defval:&str) -> Self {
        let mut me = FieldAttributes::new_nn(dt);
        me.defval = Some(defval.to_string());
        me
    }
    /// Initialize PrimaryKey FieldAttributes for this FieldType
    pub fn new_pk(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_nn(dt);
        me.primary_key = true;
        me
    }
    /// Initialize UNIQUE FieldAttributes for this FieldType
    pub fn new_uk(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_default(dt);
        me.unique = true;
        me
    }
    /// Initialize PrimaryKey+UNIQUE FieldAttributes for this FieldType
    pub fn new_uk_pk(dt:FieldType) -> Self {
        let mut me = FieldAttributes::new_uk(dt);
        me.primary_key = true;
        me
    }
    pub fn new_meta(dt:FieldType, meta:&str) -> Self {
        let mut me = FieldAttributes::new_default(dt);
        me.meta_name = Some(meta.to_string());
        me
    }
}

/// Field of a Table
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Field {
    pub name: String,
    pub attributes: FieldAttributes,
}
impl Field {
    /// Initialize Field named name with FieldAttributes
    pub fn new(name:&str, attrs:&FieldAttributes) -> Self {
        Field {
            name:name.to_string(),
            attributes:attrs.clone(),
        }
    }
    /// Initialize Field named name with FieldAttributes with only_db attributes on
    pub fn new_only_db(name:&str, attrs:&FieldAttributes) -> Self {
        let mut me = Field::new(name, attrs);
        me.attributes.only_db = true;
        me
    }
}
#[typetag::serde]
impl DBObject for Field {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        // TODO: escape all SQL reserved words
        let mut s = match self.name.as_str() {
            "role"=>format!("\"role\""),
            _=>self.name.to_owned(),
        };            
        let att = &self.attributes;
        s += format!(" {} ", att.dtype.to_sql(type_writer)).as_str();
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

/// Vector of Field's
pub type Fields = Vec<Field>;
type FieldNames = Vec<String>;

/// Types of GRANT permissions
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

/// GRANT generator
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Grant {
    pub permission: GrantType,
    pub to: String,
    pub on: ObjectPath,
}
impl Grant {
    /// Create a GRANT sepecifying permissions, grantee and affected object
    pub fn new(perm:GrantType, to:&str, on:&ObjectPath) -> Self {
        Grant { permission: perm.to_owned(), to: to.to_string(), on: on.to_owned() }
    }
}
#[typetag::serde]
impl DBObject for Grant {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        let mut rv = "".to_owned();
        if type_writer.supports_permissions() {
            if self.on.otype != ObjectType::Sequence || type_writer.supports_sequences() {
                rv = format!("GRANT {} ON {} {} TO {};", self.permission.to_string(), self.on.otype.to_string(), type_writer.schema(&self.on), self.to)
            }
        }
        rv
    }
    fn is_top_level(&self) -> bool { true }
    fn top_level_to_sql(&self, type_writer:&dyn TypeWriter, _delayed: &Vec<&Box<& dyn DBObject>>) -> String {
        self.to_sql(type_writer)
    }
}

/// Owner of a database object generator
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Owner {
    pub to: String,
    pub of: ObjectPath,
}
impl Owner {
    /// Create a Owner specifying user/role and affected object
    pub fn new(to:&str, of:&ObjectPath) -> Self {
        Owner { to: to.to_string(), of: of.to_owned() }
    }
}
#[typetag::serde]
impl DBObject for Owner {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        let mut rv = "".to_owned();
        if type_writer.supports_permissions() {
            if self.of.otype != ObjectType::Sequence || type_writer.supports_sequences() {
                rv = format!("ALTER {} {} OWNER TO {};", self.of.otype.to_string(), type_writer.schema(&self.of), self.to)
            }
        }
        rv
    }
    fn is_top_level(&self) -> bool { true }
    fn top_level_to_sql(&self, type_writer:&dyn TypeWriter, _delayed: &Vec<&Box<& dyn DBObject>>) -> String {
        self.to_sql(type_writer)
    }
}

/// INDEX generator
#[derive(Serialize, Deserialize, Debug)]
pub struct Index {
    pub table: ObjectPath,
    pub fields: FieldNames,
}
impl Index {
    pub fn new(table: &ObjectPath, fields:&FieldNames) -> Self {
        Index {
            table:table.to_owned(),
            fields:fields.to_owned(),
        }
    }
}
#[typetag::serde]
impl DBObject for Index {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        format!("CREATE INDEX {}_{}_idx ON {}{} ({});",
                self.table.name,
                self.fields.join("_"),
                type_writer.schema(&self.table),
                type_writer.index_type(),
                self.fields.join(","))
    }
	fn is_top_level(&self) -> bool { true }
}

type Indexes = Vec<Index>;

/// UniqueKey generator
#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueKey {
    pub name: String,
    pub fields: FieldNames, 
}
#[typetag::serde]
impl DBObject for UniqueKey {
    fn to_sql(&self, _type_writer:&dyn TypeWriter) -> String {
        format!("CONSTRAINT {}_uk UNIQUE ({})", self.name, self.fields.join(","))
    }
}

/// PrimaryKey generator
#[derive(Serialize, Deserialize, Debug)]
pub struct PrimaryKey {
    pub name: String,
    pub fields: FieldNames, 
}
#[typetag::serde]
impl DBObject for PrimaryKey {
    fn to_sql(&self, _type_writer:&dyn TypeWriter) -> String {
        format!("CONSTRAINT {}_pk PRIMARY KEY ({})", self.name, self.fields.join(","))
    }
}

/// Types of ForeignKey ON clause
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

/// ForeignKey generator
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

#[typetag::serde]
impl DBObject for ForeignKey {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        let (prefix, end, sep) = ("".to_string(), "", "");
        format!("{prefix}CONSTRAINT {}_{}_{}_fk{sep} FOREIGN KEY ({}){sep} REFERENCES {} ({}){sep} ON DELETE {} ON UPDATE {}{end}",
                self.table.name, self.ref_table.name, self.fields.join("_"),
                self.fields.join(","),
                type_writer.schema(&self.ref_table),
                self.ref_fields.join(","),
                self.on_delete.to_string(),
                self.on_update.to_string()
        )
    }
}

/// Types of upper-level objects
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ObjectType {
    Table,
    Sequence,
    Schema,
}
impl ToString for ObjectType {
    fn to_string(&self) -> String {
        match self {
            ObjectType::Table=>"TABLE".to_owned(),
            ObjectType::Sequence=>"SEQUENCE".to_owned(),
            ObjectType::Schema=>"SCHEMA".to_owned(),
        }
    }
}
impl Default for ObjectType {
    fn default() -> Self {
        ObjectType::Table
    }
}


/// Path of an object
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectPath {
    pub schema: Option<String>,
    pub name: String,
    #[serde(default)]
    pub otype: ObjectType,
}
impl ObjectPath {
    /// Create ObjectPath of Table on schema with name
    pub fn new_table(schema:&str, name:&str) -> Self {
        ObjectPath { schema: Some(schema.to_string()), name: name.to_string(), otype:ObjectType::Table }
    }
    /// Create ObjectPath of Schema
    pub fn new_table_only(name:&str) -> Self {
        ObjectPath { schema: None, name: name.to_string(), otype:ObjectType::Schema }
    }
    /// Create ObjectPath of a Sequence
    pub fn new_sequence(schema:&str, name:&str) -> Self {
        ObjectPath { schema: Some(schema.to_string()), name: name.to_string(), otype:ObjectType::Sequence }
    }
    /// Get the full name of this ObjectPath
    pub fn full_name(&self) -> String {
        if let Some(schema) = &self.schema {
            format!("{}.{}", schema, self.name)
        } else {
            self.name.to_owned()
        }
    }
    pub fn is_equal(&self, other:&ObjectPath) -> bool {
        let my_schema = if let Some(schema) = &self.schema {
            schema
        } else {
            ""
        };
        let other_schema = if let Some(schema) = &other.schema {
            schema
        } else {
            ""
        };
        return my_schema == other_schema
            && self.name == other.name
            && self.otype == other.otype
    }
}

/// TABLE generator
#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    pub path: ObjectPath,
    pub fields: Fields,
    pub fks: Option<ForeignKeys>,
}
impl Table {
    /// Create a table with ObjectPath and Fields
    pub fn new(path:&ObjectPath, fields:Fields, fks:Option<ForeignKeys>) -> Self {
        // lets check for duplicates
        {
            let mut unicos = HashSet::new();
            let mut dups = Vec::new();
            for f in fields.iter() {
                if ! unicos.insert(&f.name) {
                    dups.push(&f.name)
                }
            }
            if ! dups.is_empty() {                
                panic!("{} has duplicated fields: {dups:?}", path.full_name())
            }
            if let Some(thefks) = &fks {
                for fk in thefks.iter() {
                    if ! path.is_equal(&fk.table) {
                        panic!("{} is not a valid fk for {}", fk.table.full_name(), path.full_name())
                    }
                }
            }
        }
        Table {
            path: path.to_owned(),
            fields,
            fks,
        }
    }
    /// Get the indexed fields in this Table, if any
    pub fn indexes(&self) -> Option<Indexes> {
        let mut idxs:Vec<String> = Vec::new();
        for f in self.fields.iter() {
            if f.attributes.index {
                idxs.push(f.name.clone())
            }
        }
        if ! idxs.is_empty() {
            Some(vec![Index{ table:self.path.to_owned(), fields:idxs}])
        } else {
            None
        }
    }
    fn gen_sql(&self, type_writer:&dyn TypeWriter, extras:Option<&Vec<String>>) -> String {
        let cols : Vec<String> = self.fields.iter().map(|f| f.to_sql(type_writer).to_owned()).collect();
        let mut cts:Vec<Box<dyn DBObject>> = Vec::new();
        let mut uks:Vec<String> = Vec::new();
        let mut pks:Vec<String> = Vec::new();
        for f in self.fields.iter() {
            if f.attributes.primary_key {
                if type_writer.supports_auto_increment() || f.attributes.dtype != FieldType::AutoInc {
                    pks.push(f.name.to_owned())
                }                
            }
            if f.attributes.unique {
                uks.push(f.name.to_owned())
            }
        }
        if ! pks.is_empty() {
            cts.push(Box::new(PrimaryKey{ name: format!("{}_{}", self.path.name, pks.join("_")), fields:pks }))
        }
        if ! uks.is_empty() {
            cts.push(Box::new(UniqueKey{ name: format!("{}_{}", self.path.name, uks.join("_")), fields:uks}))
        }
        let mut refs : Vec<String> = cts.iter().map(|f| f.to_sql(type_writer).to_owned()).collect();
        if let Some(fks) = &self.fks {
           for fk in fks.iter() {
               refs.push(fk.to_sql(type_writer))
           }
        }
        let exts = if let Some(ext) = extras {
            format!(",\n  {}", ext.join(",\n  "))
        } else {
            "".to_owned()
        };
        let mut t=format!("CREATE TABLE {} (\n  {}{}", type_writer.schema(&self.path), cols.join(",\n  "), exts);
        if ! refs.is_empty() {
            t += format!(",\n  {}", refs.join(",\n  ")).as_str()
        }
        t += "\n);";
        t
    }
}
#[typetag::serde]
impl DBObject for Table {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        self.gen_sql(type_writer, None)
    }
    fn is_top_level(&self) -> bool { true }
    fn top_level_to_sql(&self, type_writer:&dyn TypeWriter, delayed: &Vec<&Box<& dyn DBObject>>) -> String {
        let mut extras = Vec::new();
        for obj in delayed.iter() {
            let sql = obj.to_sql(type_writer);
            // println!("sql [{sql}]");    
            if ! sql.is_empty() {
                extras.push(sql)
            }
        }
        if extras.is_empty() {
            self.gen_sql(type_writer, None)
        } else {
            self.gen_sql(type_writer, Some(&extras))
        }
    }
}

/// SCHEMA generator
#[derive(Serialize, Deserialize, Debug)]
pub struct Schema {
    pub name: String,
    pub owner: String,
}
impl Schema {
    ///  Create a new Schema
    pub fn new(name:&str, owner:&str) -> Self {
        Schema{ name: name.to_string(), owner: owner.to_string(), }
    }
}
#[typetag::serde]
impl DBObject for Schema {
    fn to_sql(&self, type_writer:&dyn TypeWriter) -> String {
        if type_writer.supports_schemas() {
            format!("CREATE SCHEMA {} AUTHORIZATION {};", self.name, self.owner)
        } else {
            "".to_owned()
        }
    }
    fn is_top_level(&self) -> bool { true }
    fn top_level_to_sql(&self, type_writer:&dyn TypeWriter, _delayed: &Vec<&Box<& dyn DBObject>>) -> String {
        self.to_sql(type_writer)
    }
}

/// List of Field definitions (for De/Serialization)
pub type DynFields = LinkedHashMap<String, FieldAttributes>;
/// Vector of ForeignKeys
pub type ForeignKeys = Vec<ForeignKey>;
