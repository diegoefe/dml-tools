extern crate serde;
extern crate serde_derive;

use dml_tools::sql::*;
use dml_tools::util::read_yaml_from_file;
use dml_tools::writers::MysqlTypeWriter;
use dml_tools::generators::SQL;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::error::Error;
use log::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct MyFields {
    pub basic: DynFields,
    pub sensitization: Option<DynFields>,
    pub foreign_keys: Option<ForeingKeys>,
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
    pub fn table_main(&self) -> String {
        format!("{}.{}", self.schema, self.tables.main)
    }
    pub fn table_cache(&self) -> String {
        format!("{}.{}", self.schema, self.tables.cache)
    }
    pub fn table_users(&self) -> String {
        format!("{}.{}", self.schema, self.tables.users)
    }
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


// use crate::cfg::{Config,SurveyConfig,Assignments};
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

fn grant_perms(gen:& mut SQL, spec:&MySpec, object:&ObjectPath) {
    gen.add(&Owner::new(&spec.roles.rw, object));
    gen.add(&Grant::new(GrantType::All, &spec.roles.rw, object));
    gen.add(&Grant::new(GrantType::All, &spec.roles.upd, object));
    gen.add(&Grant::new(GrantType::Select, &spec.roles.ro, object));
}

// WARNING: this functions ignores roster fields!
//  even though is shouldn't never be a roster in spec.basic
pub fn get_basic_assign_table_fields(spec:&MySpec) -> (Fields, MyFields) {
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
    // println!("{m_fields:#?}");
    (m_fields, a_fields)
}

fn gen_ddls(spec:&MySpec) -> Vec<String> {
    let mut gen= SQL::new(Some(Box::new(MysqlTypeWriter{})));
    let ff=&spec.fields_file;
    match read_my_fields(ff) {
        Ok(fields)=>{
            debug!("spec: {spec:#?}");
            debug!("fields: {fields:#?}");

            let schema = Schema::new(&spec.schema, &spec.roles.rw);
            gen.add(&schema);
            let oschema = ObjectPath::new_schema(&schema.name);
            gen.add(&Grant::new(GrantType::All, &spec.roles.rw, &oschema));
            gen.add(&Grant::new(GrantType::Usage, &spec.roles.upd, &oschema));
            gen.add(&Grant::new(GrantType::Usage, &spec.roles.ro, &oschema));

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
            let t_users = Table::new(&spec.path_table_users(), u_fields);
            // println!("{}", t_users);
            gen.add(&t_users);
            grant_perms(&mut gen, spec, &t_users.path);

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
            let t_cache = Table::new(&spec.path_table_cache(), c_fields);
            // println!("{}", t_cache);
            gen.add(&t_cache);
            grant_perms(&mut gen, spec, &t_cache.path);

            let o_seq = ObjectPath::new_sequence(&spec.schema, "asignaciones_cache_id_seq");
            grant_perms(&mut gen, spec, &o_seq);

            let (m_fields, a_fields) = get_basic_assign_table_fields(&spec);
            let t_main = Table::new(&spec.path_table_main(), m_fields);
            // println!("{}", t_main);
            gen.add(&t_main);
            if let Some(indexes) = t_main.indexes() {
                for index in indexes.iter() {
                    debug!("Got index: {:?}", index);
                    gen.add(index);
                }
            }
            grant_perms(&mut gen, spec, &t_main.path);

            if let Some(foreign_keys) = &a_fields.foreign_keys {
                for fk in foreign_keys.iter() {
                    if has_schema(&spec, &fk.table.schema) && has_schema(&spec, &fk.ref_table.schema) {
                        gen.add(fk);
                    } else {
                        let nfk = ForeignKey{
                            table: ObjectPath::new_table(&spec.schema, &fk.table.name),
                            fields: fk.fields.clone(),
                            ref_table: ObjectPath::new_table(&spec.schema, &fk.ref_table.name),
                            ref_fields: fk.ref_fields.clone(),
                            on_update: fk.on_update.clone(),
                            on_delete: fk.on_delete.clone(),
                        };
                        gen.add(&nfk);
                    }
                }
            }
        },
        Err(e)=>error!("Couldn't read assign spec [{ff}]: {}", e)
    }
    gen.statements().to_owned()
}

pub fn print_ddls(spec:&MySpec) {
    let sqls = gen_ddls(&spec);
    for sql in sqls.iter() {
        println!("{sql}")
    }
}

fn main() {

    // set_type_writer(Box::new(MySQLTR{}));

    let spec = MySpec{
        schema: "demo".to_owned(),
        tables: MyTables::default(),
        roles: MyRoles::default(),
        fields_file: "tests/fixtures/test-tables.yaml".to_owned()
    };
    let gen = gen_ddls(&spec);
    debug!("Generated: {gen:#?}");
    print_ddls(&spec);    
}
