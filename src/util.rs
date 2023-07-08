use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use yaml_merge_keys::*;
use std::fs::{self,File};
use log::*;
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


pub fn write_to_file(file:&str, data:&str) -> bool {
    if let Ok(mut fh) = File::create(file) {
        if fh.write_all(data.as_bytes()).is_err() {
            error!("Error while writing to '{}'", file);
            return false
        }
    } else {
        error!("Error encountered while creating file [{}]!", file);
        return false
    }
    true
}

pub fn write_yaml_to_file<I:for<'de> Serialize>(file_name:&str, doc:&I) {
    let serialized = serde_yaml::to_string(&doc).expect("To serialize doc");
    if! write_to_file(file_name, &serialized) {
        panic!("Could not write {} to '{file_name}'", serialized)
    }
}
