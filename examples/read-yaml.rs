use dml_tools::Loader;
use dml_tools::Processor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let loader = Loader::new_from_file("examples/my-dmls.yaml")?;
    let proc = Processor::new_with_objects(loader.objects(), None);
    proc.write_to_sql_file("local-my-generated.sql")?;
    Ok(())
}

