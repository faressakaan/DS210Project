use crate::csv_parser::Record;
use rand::prelude::*;
use rand::seq::SliceRandom;

pub fn sample_data(records: &[Record], sample_size: usize) -> Vec<Record> {
    let mut rng: ThreadRng = thread_rng();

    // Step 1: Filter out records without a residential_type
    let filtered_records: Vec<Record> = records
        .iter()
        .filter(|r| r.residential_type.is_some())
        .cloned()
        .collect();

    // Step 2: Sort the records by sales_ratio
    let mut sorted_records = filtered_records;
    sorted_records.sort_by(|a, b| a.sales_ratio.partial_cmp(&b.sales_ratio).unwrap());

    // Step 3: Exclude the top and bottom 150 records (outliers are affecting sales ratio, so removing some outliers will help with better analysis)
    let reduced_records = if sorted_records.len() > 300 {
        &sorted_records[150..(sorted_records.len() - 150)]
    } else {
        &sorted_records[..]
    };

    // Step 4: Randomly sample the desired number of records
    let sample_size_final = std::cmp::min(sample_size, reduced_records.len());
    reduced_records
        .choose_multiple(&mut rng, sample_size_final)
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csv_parser::Record;

    fn mock_record(residential_type: &str, sales_ratio: f64) -> Record {
        Record {
            serial_number: "X".to_string(),
            assessed_value: Some(100_000.0),
            sale_amount: Some(200_000.0),
            sales_ratio: Some(sales_ratio),
            residential_type: Some(residential_type.to_string()),
        }
    }

    #[test]
    fn test_sample_data_sorts_and_excludes_outliers() {
        let mut records = vec![];
        for i in 1..=500 {
            records.push(mock_record("Type", i as f64));
        }

        let sampled_records = sample_data(&records, 200);

        // Expected: Records in the range 151 to 350 should be present
        let mut expected_sales_ratios: Vec<f64> = (151..=350).map(|x| x as f64).collect();
        expected_sales_ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut actual_sales_ratios: Vec<f64> = sampled_records.iter().map(|r| r.sales_ratio.unwrap()).collect();
        actual_sales_ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Comparing sorted subsets of actual and expected to ensure exclusion of top and bottom 150
        assert_eq!(actual_sales_ratios, expected_sales_ratios);
    }
}
