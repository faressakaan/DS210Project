use csv::{Reader, Writer, Error};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    #[serde(rename = "Serial Number")]
    pub serial_number: String,

    #[serde(rename = "Assessed Value")]
    pub assessed_value: Option<f64>,

    #[serde(rename = "Sale Amount")]
    pub sale_amount: Option<f64>,

    #[serde(rename = "Sales Ratio")]
    pub sales_ratio: Option<f64>,

    #[serde(rename = "Residential Type")]
    pub residential_type: Option<String>,
}


#[derive(Debug, Clone)]
pub struct ClusterSummary {
    pub cluster_id: usize,
    pub avg_assessed_value: f64,
    pub avg_sales_ratio: f64,
    pub most_common_residential_type: String,
}

pub fn read_csv(file_path: &str) -> Result<Vec<Record>, Error> {
    let mut rdr = Reader::from_path(file_path)?;
    rdr.deserialize().collect()

}

pub fn write_csv(records: &[Record], file_path: &str) -> Result<(), Error> {
    let mut wtr = Writer::from_path(file_path)?;

    for record in records {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    // Helper function to create a mock Record
    fn mock_record() -> Record {
        Record {
            serial_number: "123".to_string(),
            assessed_value: Some(100000.0),
            sale_amount: Some(200000.0),
            sales_ratio: Some(0.5),
            residential_type: Some("Single Family".to_string()),
        }
    }

    #[test]
    fn test_read_csv() {
        let test_data = "Serial Number,Assessed Value,Sale Amount,Sales Ratio,Residential Type\n123,100000,200000,0.5,Single Family";
        let test_file_path = "test_read.csv";
        let mut file = File::create(test_file_path).unwrap();
        writeln!(file, "{}", test_data).unwrap();

        let records = read_csv(test_file_path).unwrap();
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].serial_number, "123");
        assert_eq!(records[0].assessed_value, Some(100000.0));

        // Clean up
        std::fs::remove_file(test_file_path).unwrap();
    }

    #[test]
    fn test_write_csv() {
        let records = vec![mock_record()];
        let test_file_path = "test_write.csv";
        write_csv(&records, test_file_path).unwrap();

        let read_records = read_csv(test_file_path).unwrap();
        assert_eq!(read_records.len(), 1);
        assert_eq!(read_records[0].serial_number, "123");
        assert_eq!(read_records[0].assessed_value, Some(100000.0));

        // Clean up
        std::fs::remove_file(test_file_path).unwrap();
    }
}


