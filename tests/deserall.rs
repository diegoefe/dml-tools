use dml_tools::Loader;
use dml_tools::Processor;
// use dml_tools::sql::*;

const SER_FILE : &str = "tests/fixtures/serialized.yaml";

#[test]
fn test_deserialization() {
/*
    let index_fields = vec!["a_field".to_string(), "another_field".to_string()];
    let index = Index::new(&ObjectPath::new_table("my_schema", "my_table"), &index_fields);

    let uk = UniqueKey{
        name: "my_unique_key".to_string(),
        fields: vec!["uk_field".to_string(), "another_uk_field".to_string()],
    };

    let pk = PrimaryKey {
        name: "my_pk".to_string(),
        fields: vec!["pk_field".to_string()],
    };

    let fk = ForeignKey {
        table: ObjectPath::new_table("schema1", "my_table"),
        fields: vec!["field1".to_string(), "field2".to_string()],
        ref_table: ObjectPath::new_table("schema1", "my_reftable"),
        ref_fields: vec!["rfield1".to_string(), "rfield2".to_string()],
        on_delete: FKOn::Restrict,
        on_update: FKOn::Cascade,
    };
*/

    let loader = Loader::new_from_file(SER_FILE).unwrap();
    let proc = Processor::new_with_objects(loader.objects(), None);

    let num_proc_objects = proc.objects().len();
    assert_eq!(num_proc_objects,loader.objects().len());
    assert_eq!(num_proc_objects, 8);
    
    // proc.add(&index);
    // proc.add(&uk);
    // proc.add(&pk);
    // proc.add(&fk);
    // proc.serialize_to_yaml_file("local-serialized.yaml").expect("to write comp file");

}