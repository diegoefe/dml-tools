use std::fs::remove_file;

use dml_tools::Loader;
use dml_tools::Processor;
use dml_tools::type_writers::Postgresql;
use dml_tools::sql::*;
use serde::{Deserialize, Serialize};
use dml_tools::util::*;

use dml_tools::macros::*;

macro_rules! des_ser_base {
    () => { "local-proc-objs" };
}

const DES_SER_FILE: &str = concat!(des_ser_base!(), ".yaml");
const DES_SER_FILE_COMP: &str = concat!(des_ser_base!(), "_comp.yaml");
const NUM_STATEMENTS: usize = 18;

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

macro_rules! grant_perms {
    ($proc:expr, $roles:expr, $opath:expr) => {
        add_owner!($proc, &($roles).rw, $opath);
        add_grant!($proc, GrantType::All, &($roles).rw, $opath);
        add_grant!($proc, GrantType::All, &($roles).upd, $opath);
        add_grant!($proc, GrantType::Select, &($roles).ro, $opath);
    }
}    

fn generate_from_code() -> Vec<String> {
    let mut proc = Processor::new(Some(Box::new(Postgresql{})));
    assert_eq!(proc.objects().len(), 0);
    let roles = MyRoles::default();
    let my_schema = String::from("my_schema");
    let schema = Schema::new(&my_schema, &roles.rw);
    proc.add(&schema);
    let oschema = ObjectPath::new_schema(&schema.name);
    
    // add_grant!(proc, g, GrantType::Usage, &roles.rw, &oschema);
    add_grant!(proc, GrantType::All, &roles.rw, &oschema);
    add_grant!(proc, GrantType::Usage, &roles.upd, &oschema);
    add_grant!(proc, GrantType::Usage, &roles.ro, &oschema);

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
    let t_users = Table::new(&ObjectPath::new_table(&my_schema, "users"), u_fields);
    // println!("{}", t_users);
    proc.add(&t_users);
    grant_perms!(&mut proc, &roles, &t_users.path);

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
        Field::new("otro", &FieldAttributes::new_meta(FieldType::Int, "4")),
    ];
    // println!("{c_fields:#?}");
    let t_cache = Table::new(&ObjectPath::new_table(&my_schema, "cache"), c_fields);
    // println!("{}", t_cache);
    proc.add(&t_cache);
    grant_perms!(&mut proc, &roles, &t_cache.path);

    let o_seq = ObjectPath::new_sequence(&my_schema, "asignaciones_cache_id_seq");
    grant_perms!(&mut proc, &roles, &o_seq);
    proc.serialize_to_yaml_file(DES_SER_FILE).expect("to write proc to file");

    proc.sql_statements()

}

// WARNING: tests get runned in alphabetic order!
#[test]
fn test_processor_from_code() {
    let sqls = generate_from_code();
    assert_eq!(sqls.join("\n"), read_file_into_string("tests/fixtures/proc.sql"));
    assert_eq!(sqls.len(), NUM_STATEMENTS);
}

#[test]
fn test_processor_from_file() {
    _ = generate_from_code();
    let loader = Loader::new_from_file(DES_SER_FILE).unwrap();
    let proc = Processor::new_with_objects(loader.objects(), None);
    assert_eq!(proc.objects().len(), NUM_STATEMENTS);
    proc.serialize_to_yaml_file(DES_SER_FILE_COMP).expect("to write comp file");
    assert_eq!(read_file_into_string(DES_SER_FILE), read_file_into_string(DES_SER_FILE_COMP));

    remove_file(DES_SER_FILE).unwrap();
    remove_file(DES_SER_FILE_COMP).unwrap();
}