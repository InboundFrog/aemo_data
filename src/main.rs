use std::error::Error;
use std::fmt;
use std::fmt::Pointer;
use log::{debug, error, info};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct YamlError<'a>('a serde_yaml::Error);

impl<'a> Error for YamlError<'a> {}

impl<'a> fmt::Display for YamlError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

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

type AemoDataResult<T> = Result<T, Box<dyn std::error::Error>>;

async fn download_aemo_yaml<T>(category: String, data_url: String) -> AemoDataResult<T> {
    info!("Fetching {} from {}", category, data_url);
    let resp = reqwest::get(&data_url).await?;

    let content_raw = resp.text().await?;
    let content_cleansed = content_raw.replace("\t", "");
    serde_yaml::from_str(&content_cleansed).map_err(|e| YamlError(e))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
            .filter(None, log::LevelFilter::Info)
            .init();
    let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

    let base_url = "https://data.wa.aemo.com.au";
    let root_data_url = format!("{base_url}/datasets/dataset-list.yaml?_={ts}", ts = ts);

    let datasets = download_aemo_yaml("datasets".to_string(), root_data_url).await?;

    info!("{:#?}", datasets);

    Ok(())
}
