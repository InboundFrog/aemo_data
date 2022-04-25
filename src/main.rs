use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DatasetEntry {
    dataset: String,
    name: String,
    description: String,
    manifest_url: String,
    definition_url: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Datasets {
    datasets: Vec<DatasetEntry>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

    let base_url = "https://data.wa.aemo.com.au";
    let root_data_url = format!("{base_url}/datasets/dataset-list.yaml?_={ts}", ts = ts);

    let resp = reqwest::get(&root_data_url).await?;

    let content_raw = resp.text().await?;
    let content_cleansed = content_raw.replace("\t", "");
    let datasets: Datasets = serde_yaml::from_str(&content_cleansed)?;

    println!("{:#?}", datasets);

    Ok(())
}
