use serde_json::Value;

pub trait TCSVWriter {
    fn write_records(&self, records: Vec<Value>) -> Result<(), Box<dyn std::error::Error>>;
}