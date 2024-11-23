use super::interfaces::t_csv_writer::TCSVWriter;
use csv::Writer;
use serde_json::Value;

#[derive(Default)]
pub struct CSVWriter {}

impl TCSVWriter for CSVWriter {
    fn write_records(
        &self,
        records: Vec<Value>,
        path: &str,
        headers: &Vec<String>,
    ) -> anyhow::Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        let mut writer = Writer::from_path(path)?;

        // Write the header row
        writer.write_record(headers);

        for video in records {
            let snippet = &video["snippet"];

            // Write each video's data to the CSV file
            writer.write_record(&[
                video["id"]["videoId"].as_str().unwrap_or(""),
                snippet["title"].as_str().unwrap_or(""),
                snippet["description"].as_str().unwrap_or(""),
                snippet["publishedAt"].as_str().unwrap_or(""),
            ])?;
        }

        // Ensure all data is written to the file
        writer.flush()?;

        Ok(())
    }
}
