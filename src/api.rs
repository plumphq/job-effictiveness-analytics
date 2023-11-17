use crate::models::*;
use reqwest;

pub async fn fetch_data_from_multiple_apis(urls: Vec<&str>) -> Vec<Result<Vec<LeverJobListingResponse>, reqwest::Error>> {
    let fetches = urls.into_iter().map(fetch_data_from_api);
    futures::future::join_all(fetches).await
}

async fn fetch_data_from_api(url: &str) -> Result<Vec<LeverJobListingResponse>, reqwest::Error> {
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                let listings = response.json::<Vec<LeverJobListingResponse>>().await?;
                println!("Fetched {} job listings from {}", listings.len(), url);
                Ok(listings)
            } else {
                println!("Failed to fetch data from {}: {}", url, response.status());
                Err(response.error_for_status().unwrap_err())
            }
        }
        Err(e) => {
            println!("Error fetching data from {}: {}", url, e);
            Err(e)
        }
    }
}
