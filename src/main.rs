use reqwest;
use serde::{Deserialize, Serialize};

use crate::api::fetch_data_from_multiple_apis;
use crate::models::FieldSizes;
use crate::utils::*;
use futures::future; // Import future module

mod models;
mod api;
mod utils;

#[tokio::main]
async fn main() {
    let urls = vec![
        "https://api.lever.co/v0/postings/superside",
        "https://api.lever.co/v0/postings/netflix",
        "https://api.lever.co/v0/postings/spotify",
        "https://api.lever.co/v0/postings/educative",
        "https://api.lever.co/v0/postings/eventbrite",
    ];

    let response_lists = fetch_data_from_multiple_apis(urls).await;

    let mut all_total_sizes = Vec::new();
    let mut all_total_token_counts = Vec::new();

    for response_list in response_lists {
        match response_list {
            Ok(data_list) => {
                let field_size_futures = data_list.iter().map(process_data);
                let field_sizes = future::join_all(field_size_futures).await;

                let total_sizes = field_sizes.iter().map(|field_size| field_size.total_size).collect::<Vec<usize>>();
                let total_stats = calculate_statistics(&total_sizes);
                println!("Total - Min: {}, Max: {}, Average: {}, Median: {}",
                         total_stats.min, total_stats.max, total_stats.average, total_stats.median);

                let total_token_counts = field_sizes.iter().map(|field_size| field_size.total_token_count).collect::<Vec<usize>>();
                let total_token_stats = calculate_statistics(&total_token_counts);
                println!("Total Tokens - Min: {}, Max: {}, Average: {}, Median: {}",
                         total_token_stats.min, total_token_stats.max, total_token_stats.average, total_token_stats.median);

                // Aggregate data for overall statistics
                all_total_sizes.extend(total_sizes);
                all_total_token_counts.extend(total_token_counts);
            }
            Err(e) => eprintln!("Error fetching data: {}", e),
        }
    }

    let overall_total_stats = calculate_statistics(&all_total_sizes);
    println!("Overall Total - Min: {}, Max: {}, Average: {}, Median: {}",
             overall_total_stats.min, overall_total_stats.max, overall_total_stats.average, overall_total_stats.median);

    let overall_token_stats = calculate_statistics(&all_total_token_counts);
    println!("Overall Tokens - Min: {}, Max: {}, Average: {}, Median: {}",
             overall_token_stats.min, overall_token_stats.max, overall_token_stats.average, overall_token_stats.median);
}
