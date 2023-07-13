# dml-tools
  [![crate](https://img.shields.io/crates/v/dml-tools.svg)](https://crates.io/crates/dml-tools)
  [![LICENSE](https://img.shields.io/crates/l/dml-tools.svg)](https://github.com/diegoefe/dml-tools/blob/main/LICENSE)
  [![docs](https://docs.rs/dml-tools/badge.svg)](https://docs.rs/dml-tools)

dml-tools is a library for DML generation and serialization from/to code or YAML files

See [examples](https://github.com/diegoefe/dml-tools/tree/main/examples) for usage.

## Install

```toml
# Cargo.toml
[dependencies]
dml-tools = "0.1"
```

## Usage
To obtain this DML (Data Manipulation Language) definitions:
  ```sql
  CREATE SCHEMA my_schema AUTHORIZATION rw_user;
  CREATE TABLE my_schema.users (
    workspace text NULL,
    user_id text NULL,
    user_name text NULL,
    full_name text NULL,
    "role" text NULL,
    is_locked text NULL,
    email text NULL,
    is_locked_by_supervisor text NULL
  );
  ALTER TABLE schema1.my_table
    ADD CONSTRAINT my_table_my_reftable_field1_field2_fk
    FOREIGN KEY (field1,field2)
    REFERENCES schema1.my_reftable (rfield1,rfield2)
    ON DELETE RESTRICT ON UPDATE CASCADE;
  ```

One can either, load it from this YAML file:
  ```yaml
  - tag: Schema
    name: my_schema
    owner: rw_user
  - tag: Table
    path:
      schema: my_schema
      name: users
      otype: Table
    fields:
    - name: workspace
      attributes:
      unique: true
      primary_key: true
    - name: user_id
      attributes:
      empty: false
      primary_key: true
    - name: user_name
      attributes:
      unique: true
    - name: full_name
      attributes: {}
    - name: role
      attributes:
      empty: false
    - name: is_locked
      attributes:
      type: bool
      empty: false
    - name: email
      attributes: {}
    - name: is_locked_by_supervisor
      attributes:
      type: bool
      empty: false
      defval: 'false'
  - tag: ForeignKey
    table:
      schema: schema1
      name: my_table
      otype: Table
    fields:
    - field1
    - field2
    ref_table:
      schema: schema1
      name: my_reftable
      otype: Table
    ref_fields:
    - rfield1
    - rfield2
    on_delete: Restrict
    on_update: Cascade
  ```
  
  with this code:
  ```rust
  use dml_tools::Loader;
  use dml_tools::Processor;
  use std::error::Error;
  use std::fs::File;
  use std::io::prelude::*;

  fn main() -> Result<(), Box<dyn Error>> {
      let loader = Loader::new("examples/my-dmls.yaml")?;
      // by default, it generates PostgreSQL DML statements
      let proc = Processor::new_with_objects(loader.objects(), None);
      proc.write_to_sql_file("my-generated.sql")?;
      Ok(())
  }
  ```
  
  Or, generate it with this code:
  ```rust
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

  macro_rules! grant_perms {
    ($proc:expr, $roles:expr, $opath:expr) => {
        add_owner!($proc, &($roles).rw, $opath);
        add_grant!($proc, GrantType::All, &($roles).rw, $opath);
        add_grant!($proc, GrantType::All, &($roles).upd, $opath);
        add_grant!($proc, GrantType::Select, &($roles).ro, $opath);
    }
  }

  fn main() -> Result<(), Box<dyn Error>> {
    let mut processor= Processor::new(Some(Box::new(dml_tools::type_writers::Mysql{})));

    let roles = MyRoles::default();
    let my_schema = String::from("my_schema");
    let schema = Schema::new(&my_schema, &roles.rw);
    processor.add(&schema);
    let oschema = ObjectPath::new_schema(&schema.name);

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
    // println!("{}", t_users);
    processor.add(&t_users);
    grant_perms!(&mut processor, roles, &t_users.path);

    processor.serialize_to_yaml_file("my-generated.sql")?;

    Ok(())
  }
```

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or <http://opensource.org/licenses/MIT>)

