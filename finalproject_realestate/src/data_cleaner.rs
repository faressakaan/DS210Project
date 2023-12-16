use crate::csv_parser::Record;

pub fn clean_data(records: &[Record]) -> Vec<Record> {
    records.iter()
        .filter(|rec| rec.residential_type.is_some())
        .cloned()
        .collect()
}
