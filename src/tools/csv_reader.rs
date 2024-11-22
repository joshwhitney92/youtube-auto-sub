use std::fs::File;
use super::interfaces::t_csv_reader::TCSVReader;

#[derive(Default)]
pub struct CSVReader {

}

impl TCSVReader for CSVReader {
    fn read_records<'a, T>(file: &File) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned {
            let mut records: Vec<T> = Vec::new();
            let mut rdr = csv::Reader::from_reader(file);
            for result in rdr.deserialize() {
                let record: T = result?;
                records.push(record);
            }

            Ok(records)
    }
}


