mod csv_parser;
mod data_cleaner;
mod sampler;
mod graph;

fn main() {
    let read_file_path = "/Users/faressakaan/Desktop/Real_Estate_Sales_2001-2020_GL.csv";
    let write_file_path = "/Users/faressakaan/Desktop/realestatesampled10k.csv";
    let records = csv_parser::read_csv(read_file_path).expect("Failed to read CSV");

    let cleaned_records = data_cleaner::clean_data(&records);
    let sampled_records = sampler::sample_data(&cleaned_records, 9_700);
    
    // Initialize the graph with a similarity threshold for sales ratio
    let similarity_threshold = 0.05; // 5% similarity threshold for sales ratio
    let mut property_graph = graph::PropertyGraph::new(similarity_threshold);

    // Create bins and assign nodes to clusters based on assessed value
    property_graph.create_bins(&sampled_records, 10);

    // Analyze each cluster
    property_graph.analyze_clusters();

    // Write the adjusted data to a new CSV file
    csv_parser::write_csv(&sampled_records, write_file_path).expect("Failed to write CSV");
}
