use dml_tools::sql::*;
use dml_tools::spec::*;
use dml_tools::util::write_yaml_to_file;

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