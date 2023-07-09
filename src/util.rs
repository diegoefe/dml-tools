use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use yaml_merge_keys::*;
use std::fs::{self,File};
use std::io::prelude::*;

pub fn read_yaml_from_string<T:for<'de> Deserialize<'de> >(str: &str) -> Result<T, Box<dyn Error>> {
    use yaml_merge_keys::serde_yaml::Value;
    let sy:Value = serde_yaml::from_str(str)?;
    let v: Result<Value, serde_yaml::Error> = serde_yaml::to_value(sy);
    
    let fix: Result<Value, MergeKeyError> = merge_keys_serde(v?);
    Ok(serde_yaml::from_value(fix?)?)
}

pub fn read_yaml_from_file<T:for<'de> Deserialize<'de>, P: AsRef<Path>>(path: P) -> Result<T, Box<dyn Error>> {
    let sfile:String = fs::read_to_string(path)?.parse()?;
    read_yaml_from_string(&sfile)
}

pub fn read_file_into_string(file:&str) -> String {
    fs::read_to_string(file).expect(format!("To read '{file}' into string").as_str())
}

pub fn write_yaml_to_file<I:for<'de> Serialize>(file_name:&str, doc:&I)  -> Result<(), Box<dyn Error>> {
    let serialized = serde_yaml::to_string(&doc)?;
    let mut fh = File::create(file_name)?;
    fh.write_all(&serialized.as_bytes())?;
    Ok(())
}
