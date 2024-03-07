use dml_tools::sql::*;
use dml_tools::Processor;
use std::error::Error;
use dml_tools::macros::*;

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut processor= Processor::new(Some(Box::new(dml_tools::type_writers::Mysql{})));

    let roles = MyRoles::default();
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
        Field::new("user_name", &FieldAttributes::new_uk(FieldType::Txt)),
        Field::new("full_name", &FieldAttributes::new(FieldType::Txt)),
        Field::new("role", &FieldAttributes::new_nn(FieldType::Txt)),
        Field::new("is_locked_by_supervisor", &FieldAttributes::new_nn_def(FieldType::Bool, "false")),
    ];

    let t_users = Table::new(&ObjectPath::new_table(&my_schema, "users"), u_fields);
    processor.add(&t_users);
    grant_perms!(&mut processor, roles, &t_users.path);

    processor.write_to_sql_file("local-dmls.sql")?;

    Ok(())

}
