use dml_tools::sql::*;
use dml_tools::util::read_yaml_from_file;
use dml_tools::Processor;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::error::Error;
use dml_tools::macros::*;

use log::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MyFields {
    pub basic: DynFields,
    pub sensitization: Option<DynFields>,
    pub foreign_keys: Option<ForeignKeys>,
}

pub fn read_my_fields<P: AsRef<Path>>(path: P) -> Result< MyFields, Box<dyn Error>> {
    Ok(read_yaml_from_file(path)?)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyRoles {
    pub rw: String,
    pub ro: String,
    pub upd: String,
}
impl Default for MyRoles {
    fn default() -> Self {
        MyRoles {
            rw: "rw_user".into(),
            ro: "ro_user".into(),
            upd: "upd_user".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyTables {
    main: String,
    cache: String,
    users: String,
}
impl Default for MyTables {
    fn default() -> Self {
        MyTables {
            main: "asignaciones".into(),
            cache: "asignaciones_cache".into(),
            users: "users".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MySpec {
    pub schema: String,
    #[serde(default)]
    pub tables: MyTables,
    #[serde(default)]
    pub roles: MyRoles,
    pub fields_file: String, // points to MyFields aware file
}
impl MySpec {
    pub fn path_table_main(&self) -> ObjectPath {
        ObjectPath::new_table(&self.schema, &self.tables.main)
    }
    pub fn path_table_cache(&self) -> ObjectPath {
        ObjectPath::new_table(&self.schema, &self.tables.cache)
    }
    pub fn path_table_users(&self) -> ObjectPath {
        ObjectPath::new_table(&self.schema, &self.tables.users)
    }
}

#[allow(dead_code)]
fn has_schema(spec:&MySpec, oschema:&Option<String>) -> bool {
    if let Some(schema) = oschema {
        if *schema != spec.schema {
            panic!("Invalid schema: {}", schema)
        }
        true
    } else {
        false
    }
}

fn gen_ddls(spec:&MySpec) -> Vec<String> {
    let ff=&spec.fields_file;
    let mut sqls=Vec::new();
    match read_my_fields(ff) {
        Ok(fields)=>{
            let indexes;
            let mut fks=Vec::new();
            {
                // let mut processor= Processor::new(Some(Box::new(dml_tools::type_writers::Sqlite{})));
                let mut processor= Processor::new(None); // Postgres
                // let mut processor= Processor::new(None);
                debug!("spec: {spec:#?}");
                debug!("fields: {fields:#?}");
    
                let roles = &spec.roles;
                let my_schema = String::from("my_schema");
                let schema = Schema::new(&my_schema, &roles.rw);
                processor.add(&schema);
                let oschema = ObjectPath::new_table_only(&schema.name);
            
                add_grant!(processor, GrantType::All, &roles.rw, &oschema);
                add_grant!(processor, GrantType::Usage, &roles.upd, &oschema);
                add_grant!(processor, GrantType::Usage, &roles.ro, &oschema);
    
                let u_fields = vec![
                    Field::new("workspace", &FieldAttributes::new_uk_pk(FieldType::Txt)),
                    Field::new("is_archived", &FieldAttributes::new(FieldType::Bool)),
                    Field::new("creation_date", &FieldAttributes::new_nn(FieldType::Txt)),
                    Field::new("user_id", &FieldAttributes::new_pk(FieldType::Txt)),
                    Field::new("user_name", &FieldAttributes::new_uk(FieldType::Txt)),
                    Field::new("full_name", &FieldAttributes::new(FieldType::Txt)),
                    Field::new("role", &FieldAttributes::new_nn(FieldType::Txt)),
                    Field::new("is_locked", &FieldAttributes::new_nn(FieldType::Bool)),
                    Field::new("email", &FieldAttributes::new(FieldType::Txt)),
                    Field::new("phone_number", &FieldAttributes::new(FieldType::Txt)),
                    Field::new("supervisor_id", &FieldAttributes::new(FieldType::Txt)),
                    Field::new("is_locked_by_supervisor", &FieldAttributes::new_nn_def(FieldType::Bool, "false")),
                    Field::new("is_locked_by_headquarters", &FieldAttributes::new_nn_def(FieldType::Bool, "false")),
                ];
                // println!("{u_fields:#?}");
                let t_users = Table::new(&spec.path_table_users(), u_fields, None);
                // println!("{}", t_users);
                processor.add(&t_users);
                grant_perms!(&mut processor, &spec.roles, &t_users.path);
    
                let c_fields = vec![
                    Field::new("id", &FieldAttributes::new_nn(FieldType::AutoInc)),
                    Field::new("pk", &FieldAttributes::new(FieldType::Txt)),
                    Field::new("responsible", &FieldAttributes::new_nn(FieldType::Txt)),
                    Field::new("responsible_id", &FieldAttributes::new_nn(FieldType::Txt)),
                    Field::new("workspace", &FieldAttributes::new_pk(FieldType::Txt)),
                    Field::new("survey_id", &FieldAttributes::new_pk(FieldType::Int)),
                    Field::new("questionnaire_id", &FieldAttributes::new_nn(FieldType::Txt)),
                    Field::new("date_created", &FieldAttributes::new_nn(FieldType::Txt)),
                    Field::new("date_updated", &FieldAttributes::new_nn(FieldType::Txt)),
                ];
                // println!("{c_fields:#?}");
                let t_cache = Table::new(&spec.path_table_cache(), c_fields, None);
                // println!("{}", t_cache);
                processor.add(&t_cache);
                grant_perms!(&mut processor, &spec.roles, &t_cache.path);
    
                let o_seq = ObjectPath::new_sequence(&spec.schema, "asignaciones_cache_id_seq");
                grant_perms!(&mut processor, &spec.roles, &o_seq);
    
                // let (m_fields, a_fields) = get_basic_assign_table_fields(&spec);
                let mut m_fields = vec![
                    Field::new_only_db("interview_id", &FieldAttributes::new_pk(FieldType::BigInt)),
                    Field::new_only_db("responsible", &FieldAttributes::new_nn(FieldType::Txt)),
                    Field::new_only_db("workspace", &FieldAttributes::new_nn_idx(FieldType::Txt)),
                    Field::new_only_db("ts", &FieldAttributes::new_nn_idx(FieldType::Txt)),
                    Field::new_only_db("habilitado", &FieldAttributes::new_nn_def(FieldType::Bool, "true")),
                    Field::new_only_db("survey_id", &FieldAttributes::new(FieldType::Int)),
                    Field::new_only_db("agregado", &FieldAttributes::new_nn_def(FieldType::Bool, "false")),
                ];
                let a_fields = read_my_fields(&spec.fields_file).expect(format!("Read fields from '{}'", spec.fields_file).as_str());
                for (name, attrs) in a_fields.basic.iter() {
                    if ! attrs.roster {
                        m_fields.push(Field{ name:name.to_owned(), attributes:attrs.to_owned()})
                    }
                }
                if let Some(foreign_keys) = &a_fields.foreign_keys {
                    for fk in foreign_keys.iter() {
                        if has_schema(&spec, &fk.table.schema) && has_schema(&spec, &fk.ref_table.schema) {
                            processor.add(fk);
                        } else {
                            let nfk = ForeignKey{
                                table: ObjectPath::new_table(&spec.schema, &fk.table.name),
                                fields: fk.fields.clone(),
                                ref_table: ObjectPath::new_table(&spec.schema, &fk.ref_table.name),
                                ref_fields: fk.ref_fields.clone(),
                                on_update: fk.on_update.clone(),
                                on_delete: fk.on_delete.clone(),
                            };
                            fks.push(nfk);
                        }
                    }
                    // if ! fks.is_empty() {
                    //     for fk in fks.iter() {
                    //         processor.add(fk);
                    //     }
                    // }
                }
                let the_fks = if fks.is_empty() {
                   None
                } else {
                   Some(fks)
                };
                let t_main = Table::new(&spec.path_table_main(), m_fields, the_fks);
                // println!("{}", t_main);
                processor.add(&t_main);

                indexes = if let Some(idxs) = t_main.indexes() {
                    idxs
                } else {
                    Vec::new()
                };
                if !indexes.is_empty() {
                    for index in indexes.iter() {
                        debug!("Got index: {:?}", index);
                        processor.add(index);
                    }
                }
                grant_perms!(&mut processor, &spec.roles, &t_main.path);
                sqls = processor.sql_statements()
            }
        },
        Err(e)=>error!("Couldn't read assign spec [{ff}]: {}", e)
    }
    sqls
}

pub fn print_ddls(spec:&MySpec) {
    let sqls = gen_ddls(&spec);
    for sql in sqls.iter() {
        println!("{sql}")
    }
}

fn main() {
    let spec = MySpec{
        schema: "demo".to_owned(),
        tables: MyTables::default(),
        roles: MyRoles::default(),
        fields_file: "tests/fixtures/test-tables.yaml".to_owned()
    };
    let dlls = gen_ddls(&spec);
    debug!("Generated: {dlls:#?}");
    print_ddls(&spec);    
}
