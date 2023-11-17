use scraper::{Html, Selector};
use crate::models::*;
use tiktoken_rs::p50k_base;
use tokio::task;

pub async fn process_data(data: &LeverJobListingResponse) -> FieldSizes {
    let mut combined_text = String::new();
    combined_text.push_str(&data.id);
    combined_text.push_str(&data.description_plain);
    combined_text.push_str(&data.text);
    combined_text.push_str(&data.country.as_ref().unwrap_or(&String::new()));
    combined_text.push_str(&data.workplace_type.as_ref().unwrap_or(&String::new()));

    if data.categories.is_some() {
        combined_text.push_str(&data.categories.as_ref().unwrap().commitment.as_ref().unwrap_or(&String::new()));
        combined_text.push_str(&data.categories.as_ref().unwrap().department.as_ref().unwrap_or(&String::new()));
        combined_text.push_str(&data.categories.as_ref().unwrap().location.as_ref().unwrap_or(&String::new()));
        combined_text.push_str(&data.categories.as_ref().unwrap().team.as_ref().unwrap_or(&String::new()));
        if data.categories.as_ref().unwrap().all_locations.is_some() {
            for location in data.categories.as_ref().unwrap().all_locations.as_ref().unwrap() {
                combined_text.push_str(&location);
            }
        }
    }

    if data.lists.is_some() {
        for list_item in data.lists.as_ref().unwrap() {
            combined_text.push_str(&list_item.text);
            combined_text.push_str(&list_item.content);
        }
    }

    let cleaned_combined_text = clean_html_content(&combined_text);
    let text_for_tokenization = cleaned_combined_text.clone();

    let tokens = task::spawn_blocking(move || {
        p50k_base().unwrap().encode_with_special_tokens(&text_for_tokenization)
    }).await.unwrap(); // Handle errors appropriately

    FieldSizes {
        total_size: cleaned_combined_text.len(),
        total_token_count: tokens.len(),
    }
}

fn clean_html_content(html_content: &str) -> String {
    let document = Html::parse_document(html_content);
    let selector = Selector::parse("body").unwrap();
    document.select(&selector).next().map_or(String::new(), |n| n.text().collect())
}

pub fn calculate_statistics(lengths: &[usize]) -> Statistics {
    let min = *lengths.iter().min().unwrap_or(&0);
    let max = *lengths.iter().max().unwrap_or(&0);
    let sum: usize = lengths.iter().sum();
    let count = lengths.len();
    let average = sum as f32 / count as f32;
    let median = calculate_median(lengths);

    Statistics { min, max, average, median }
}

fn calculate_median(lengths: &[usize]) -> usize {
    if lengths.is_empty() {
        return 0;
    }

    let mut sorted = lengths.to_vec();
    sorted.sort_unstable();

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2
    } else {
        sorted[mid]
    }
}
