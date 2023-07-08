use dml_tools::sql::*;

mod common;
use common::*;

#[test]
fn test_fields() {
    let fields = read_test_tables("tests/fixtures/test-tables.yaml").expect("to open test-tables.yaml");
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
