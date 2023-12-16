use crate::csv_parser::Record;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Cluster {
    pub properties: Vec<Record>,
}

impl Cluster {
    pub fn new() -> Self {
        Cluster {
            properties: Vec::new(),
        }
    }
}

pub struct PropertyGraph {
    pub clusters: HashMap<usize, Cluster>, // Clusters identified by a bin number
    pub similarity_threshold: f64,         // Additional similarity threshold for sales ratio
}

impl PropertyGraph {
    pub fn new(similarity_threshold: f64) -> Self {
        PropertyGraph {
            clusters: HashMap::new(),
            similarity_threshold,
        }
    }

    // Creates bins based on assessed values and assigns properties to clusters
    pub fn create_bins(&mut self, records: &[Record], number_of_bins: usize) {
        let mut values: Vec<f64> = records.iter()
            .filter_map(|r| r.assessed_value)
            .collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let mut bin_ranges: Vec<(f64, f64)> = vec![(f64::MAX, f64::MIN); number_of_bins];

        // Create quantile-based bin edges to handle outliers
        let bin_edges: Vec<f64> = (1..number_of_bins).map(|i| values[i * values.len() / number_of_bins]).collect();

        for record in records {
            if let Some(value) = record.assessed_value {
                // Find the bin index using the bin edges, defaulting to the last bin if value is higher than all edges
                let bin_index = bin_edges.iter().rposition(|&edge| value >= edge).unwrap_or(number_of_bins - 1);
                self.assign_to_cluster(record.clone(), bin_index);
                // Update the min and max values for the bin
                bin_ranges[bin_index].0 = bin_ranges[bin_index].0.min(value);
                bin_ranges[bin_index].1 = bin_ranges[bin_index].1.max(value);
            }
        }

        // Print the range for each bin
        for (i, range) in bin_ranges.iter().enumerate() {
            if self.clusters.get(&i).is_some() {
                println!("Bin {}: Range {} - {}", i, range.0, range.1);
            }
        }


        // Debugging: Print the number of records in each bin
        for i in 0..number_of_bins {
            println!("Bin {}: {} records", i, self.clusters.get(&i).map_or(0, |cluster| cluster.properties.len()));
        }

        
    }

    // Assigns a property to a cluster
    pub fn assign_to_cluster(&mut self, record: Record, bin_index: usize) {
        self.clusters.entry(bin_index).or_insert_with(Cluster::new).properties.push(record);
    }


    // Analyzes clusters and calculates statistics
    pub fn analyze_clusters(&self) {
        for (cluster_id, cluster) in &self.clusters {
            let avg_assessed_value = cluster.properties.iter().filter_map(|r| r.assessed_value).sum::<f64>() / cluster.properties.len() as f64;
            let avg_sales_ratio = cluster.properties.iter().filter_map(|r| r.sales_ratio).sum::<f64>() / cluster.properties.len() as f64;
            let residential_type_counts = cluster.properties.iter().map(|r| r.residential_type.clone()).fold(HashMap::new(), |mut acc, rt| {
                *acc.entry(rt).or_insert(0) += 1;
                acc
            });
            let mode_residential_type = residential_type_counts.into_iter().max_by_key(|&(_, count)| count).map(|(rt, _)| rt);

            println!("Cluster {}: Avg Assessed Value: {:.2}, Avg Sales Ratio: {:.2}, Most Common Residential Type: {:?}", cluster_id, avg_assessed_value, avg_sales_ratio, mode_residential_type);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Create a mock Record for testing purposes
    fn mock_record(serial_number: &str, assessed_value: f64, sales_ratio: f64, residential_type: &str) -> Record {
        Record {
            serial_number: serial_number.to_string(),
            assessed_value: Some(assessed_value),
            sale_amount: Some(100000.0),  // Assuming there is a sale_amount field
            sales_ratio: Some(sales_ratio),
            residential_type: Some(residential_type.to_string()),
        }
    }

    #[test]
    fn test_create_bins() {
        let mut graph = PropertyGraph::new(0.05);
        let records = vec![
            mock_record("1", 100000.0, 0.8, "Single Family"),
            mock_record("2", 200000.0, 0.9, "Single Family"),
            // ... more mock records ...
        ];
        graph.create_bins(&records, 2);
        assert_eq!(graph.clusters.len(), 2);
        // ... additional assertions ...
    }

    #[test]
    fn test_assign_to_cluster() {
        let mut graph = PropertyGraph::new(0.05);
        let record = mock_record("1", 100000.0, 0.8, "Single Family");
        graph.assign_to_cluster(record, 0);
        assert_eq!(graph.clusters.get(&0).unwrap().properties.len(), 1);
        // ... additional assertions ...
    }

    #[test]
    fn test_analyze_clusters() {
        let mut graph = PropertyGraph::new(0.05);
        let records = vec![
            mock_record("1", 100000.0, 0.8, "Single Family"),
            mock_record("2", 200000.0, 0.9, "Condo"),
            // ... more mock records ...
        ];
        graph.create_bins(&records, 2);
        graph.analyze_clusters();
        // Since analyze_clusters does not return anything and just prints,
        // you might need to capture stdout or modify the method to allow testing.
    }
}

