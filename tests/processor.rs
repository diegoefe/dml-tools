use dml_tools::Processor;
use dml_tools::type_writers::Postgresql;
use dml_tools::sql::*;
use serde::{Deserialize, Serialize};
use dml_tools::util::*;

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


fn grant_perms(processor:& mut Processor, roles:&MyRoles, object:&ObjectPath) {
    processor.add(&Owner::new(&roles.rw, object));
    processor.add(&Grant::new(GrantType::All, &roles.rw, object));
    processor.add(&Grant::new(GrantType::All, &roles.upd, object));
    processor.add(&Grant::new(GrantType::Select, &roles.ro, object));
}

#[test]
fn test_processor() {
    let mut proc = Processor::new(Some(Box::new(Postgresql{})));
    assert_eq!(proc.sql_statements().len(), 0);
    let roles = MyRoles::default();
    let my_schema = String::from("my_schema");
    let schema = Schema::new(&my_schema, &roles.rw);
    proc.add(&schema);
    let oschema = ObjectPath::new_schema(&schema.name);
    proc.add(&Grant::new(GrantType::All, &roles.rw, &oschema));
    proc.add(&Grant::new(GrantType::Usage, &roles.upd, &oschema));
    proc.add(&Grant::new(GrantType::Usage, &roles.ro, &oschema));

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
    grant_perms(&mut proc, &roles, &t_users.path);

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
    let t_cache = Table::new(&ObjectPath::new_table(&my_schema, "cache"), c_fields);
    // println!("{}", t_cache);
    proc.add(&t_cache);
    grant_perms(&mut proc, &roles, &t_cache.path);

    let o_seq = ObjectPath::new_sequence(&my_schema, "asignaciones_cache_id_seq");
    grant_perms(&mut proc, &roles, &o_seq);
    // println!("{}", proc.to_string());
    assert_eq!(proc.to_string(), read_file_into_string("tests/fixtures/proc.sql"));
    assert_eq!(proc.sql_statements().len(), 18);

}