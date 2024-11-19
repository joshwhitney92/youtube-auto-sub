use csv::Writer;
use serde_json::Value;
use super::interfaces::t_csv_writer::TCSVWriter;


pub struct CSVWriter<'a> {
    output_path: String,
    headers: &'a Vec<String>
}

impl<'a> CSVWriter<'a> {
    pub fn new(path: &str, headers: &'a Vec<String>) -> Self {
        Self { output_path: path.to_owned(), headers }
    }
}

impl<'a> TCSVWriter for CSVWriter<'a> {
    fn write_records(&self, records: Vec<Value>) -> Result<(), Box<dyn std::error::Error>> {
        // Create a new CSV writer and specify the output file name.
        let mut writer = Writer::from_path(&self.output_path)?;

        // Write the header row
        writer.write_record(self.headers);

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
