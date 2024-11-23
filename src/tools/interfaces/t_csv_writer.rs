use serde_json::Value;

pub trait TCSVWriter {
    fn write_records(
        &self,
        records: Vec<Value>,
        path: &str,
        headers: &[String],
    ) -> anyhow::Result<(), Box<dyn std::error::Error>>;
}
