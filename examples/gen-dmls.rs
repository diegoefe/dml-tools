extern crate serde;
extern crate serde_derive;

use dml_tools::sql::*;
use dml_tools::spec::*;
use log::*;

// use crate::cfg::{Config,SurveyConfig,Assignments};
fn has_schema(spec:&AssignSpec, oschema:&Option<String>) -> bool {
    if let Some(schema) = oschema {
        if *schema != spec.schema {
            panic!("Invalid schema: {}", schema)
        }
        true
    } else {
        false
    }
}

fn grant_perms(sqls:&mut Vec<String>, spec:&AssignSpec, object:&ObjectPath) {
    sqls.push(Owner::new(&spec.roles.rw, object).to_sql());
    sqls.push(Grant::new(GrantType::All, &spec.roles.rw, object).to_sql());
    sqls.push(Grant::new(GrantType::All, &spec.roles.upd, object).to_sql());
    sqls.push(Grant::new(GrantType::Select, &spec.roles.ro, object).to_sql());
}

// WARNING: this functions ignores roster fields!
//  even though is shouldn't never be a roster in spec.basic
pub fn get_basic_assign_table_fields(spec:&AssignSpec) -> (Fields, AsgFields) {
    let mut m_fields = vec![
        Field::new_only_db("interview_id", &FieldAttributes::new_pk(FieldType::BigInt)),
        Field::new_only_db("responsible", &FieldAttributes::new_nn(FieldType::Txt)),
        Field::new_only_db("workspace", &FieldAttributes::new_nn_idx(FieldType::Txt)),
        Field::new_only_db("ts", &FieldAttributes::new_nn_idx(FieldType::Txt)),
        Field::new_only_db("habilitado", &FieldAttributes::new_nn_def(FieldType::Bool, "true")),
        Field::new_only_db("survey_id", &FieldAttributes::new(FieldType::Int)),
        Field::new_only_db("agregado", &FieldAttributes::new_nn_def(FieldType::Bool, "false")),
    ];
    let a_fields = read_asg_fields(&spec.fields_file).expect(format!("Read fields from '{}'", spec.fields_file).as_str());
    for (name, attrs) in a_fields.basic.iter() {
        if ! attrs.roster {
            m_fields.push(Field{ name:name.to_owned(), attributes:attrs.to_owned()})
        }
    }
    // println!("{m_fields:#?}");
    (m_fields, a_fields)
}

fn gen_ddls(spec:&AssignSpec) -> Vec<String> {
    let mut sqls = Vec::new();
    let ff=&spec.fields_file;
    match read_asg_fields(ff) {
        Ok(fields)=>{
            debug!("spec: {spec:#?}");
            debug!("fields: {fields:#?}");

            let schema = Schema::new(&spec.schema, &spec.roles.rw);
            sqls.push(schema.to_sql());
            let oschema = ObjectPath::new_schema(&schema.name);
            sqls.push(Grant::new(GrantType::All, &spec.roles.rw, &oschema).to_sql());
            sqls.push(Grant::new(GrantType::Usage, &spec.roles.upd, &oschema).to_sql());
            sqls.push(Grant::new(GrantType::Usage, &spec.roles.ro, &oschema).to_sql());

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
            // println!("{}", t_users.to_sql());
            sqls.push(t_users.to_sql());
            grant_perms(&mut sqls, spec, &t_users.path);

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
            // println!("{}", t_cache.to_sql());
            sqls.push(t_cache.to_sql());
            grant_perms(&mut sqls, spec, &t_cache.path);

            let o_seq = ObjectPath::new_sequence(&spec.schema, "asignaciones_cache_id_seq");
            grant_perms(&mut sqls, spec, &o_seq);

            let (m_fields, a_fields) = get_basic_assign_table_fields(&spec);
            let t_main = Table::new(&spec.path_table_main(), m_fields);
            // println!("{}", t_main.to_sql());
            sqls.push(t_main.to_sql());
            if let Some(indexes) = t_main.indexes() {
                for index in indexes.iter() {
                    debug!("Got index: {:?}", index);
                    sqls.push(index.to_sql())
                }
            }
            grant_perms(&mut sqls, spec, &t_main.path);

            if let Some(fks) = &a_fields.fks {
                for fk in fks.iter() {
                    if has_schema(&spec, &fk.table.schema) && has_schema(&spec, &fk.ref_table.schema) {
                        sqls.push(fk.to_sql())
                    } else {
                        let nfk = ForeignKey{
                            table: ObjectPath::new_table(&spec.schema, &fk.table.name),
                            fields: fk.fields.clone(),
                            ref_table: ObjectPath::new_table(&spec.schema, &fk.ref_table.name),
                            ref_fields: fk.ref_fields.clone(),
                            on_update: fk.on_update.clone(),
                            on_delete: fk.on_delete.clone(),
                        };
                        sqls.push(nfk.to_sql())
                    }
                }
            }
        },
        Err(e)=>error!("Couldn't read assign spec [{ff}]: {}", e)
    }
    sqls
}

pub fn print_ddls(spec:&AssignSpec) {
    let sqls = gen_ddls(&spec);
    for sql in sqls.iter() {
        println!("{sql}")
    }
}

fn main() {
    let spec = AssignSpec{
        schema: "demo".to_owned(),
        tables: AsgTables::default(),
        roles: AsgRoles::default(),
        fields_file: "fixtures/test-fields.yaml".to_owned()
    };
    let gen = gen_ddls(&spec);
    debug!("Generated: {gen:#?}");
    print_ddls(&spec);    
}
