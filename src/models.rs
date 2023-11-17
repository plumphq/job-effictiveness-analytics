use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeverJobListingResponse {
    pub id: String,
    pub categories: Option<LeverCategories>,
    #[serde(rename = "descriptionPlain")]
    pub description_plain: String,
    pub lists: Option<Vec<LeverListItem>>,
    pub text: String,
    pub country: Option<String>,
    #[serde(rename = "workplaceType")]
    pub workplace_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeverCategories {
    pub commitment: Option<String>,
    pub department: Option<String>,
    pub location: Option<String>,
    pub team: Option<String>,
    #[serde(rename = "allLocations")]
    pub all_locations: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeverListItem {
    pub text: String,
    pub content: String,
}

#[derive(Default)]
pub struct FieldSizes {
    pub total_size: usize,
    pub total_token_count: usize,
}

#[derive(Default)]
pub struct Statistics {
    pub min: usize,
    pub max: usize,
    pub average: f32,
    pub median: usize,
}