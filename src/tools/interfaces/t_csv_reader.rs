use std::fs::File;
use serde::de::DeserializeOwned;

pub trait TCSVReader {
    fn read_records<'a, T>(file: &File) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned;
}
