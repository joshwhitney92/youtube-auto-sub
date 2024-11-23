use serde::de::DeserializeOwned;
use std::fs::File;

pub trait TCSVReader {
    fn read_records<'a, T>(file: &File) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned;
}
