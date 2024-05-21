use anyhow::Error;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use gloo_timers::future::TimeoutFuture;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceConfig {
    pub user_id: i32,
    pub hostname: String,
    pub config: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UploadConfig {
    config_content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceInfo {
    pub user_id: i32,
    pub device_hostname: String,
    pub location: String,
    pub client_name: String,
    pub device_type: String,
    pub config_name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct ConfigResponse {
    pub success: bool,
    pub message: String,
    pub config_id: i32,
    pub storage_location: String,
    pub shared_link: String,
    pub access_key: String,
}

/// Sends the device configuration to the server which decides where to store it based on the `use_cloud_storage` flag.
///
/// # Arguments
/// * `server_uri` - The base URI of the server handling the storage.
/// * `device_config` - The device configuration data.
/// * `use_cloud_storage` - Boolean indicating whether to use cloud storage or local storage.
///
/// # Returns
/// A `Result` which is `Ok` with a success message if the configuration was sent successfully, or an `Error` if an error occurred.
pub async fn send_config_to_server(server_uri: &str, config_id: i32, device_config: &DeviceConfig, storage_location: &str, api_key: &Option<String>) -> Result<String, Error> {
    let endpoint = match storage_location {
        "cloud" => "upload_cloud",
        _ => "upload_local",
    };

    let api_key_ref = api_key.as_deref().ok_or_else(|| anyhow::Error::msg("API key is missing"))?;

    // Prepare the payload with config content
    let upload_config = UploadConfig {
        config_content: device_config.config.clone(),
    };
    let json_body = serde_json::to_string(&upload_config)?;

    let url = format!("{}/api/data/{}/{}", server_uri, endpoint, config_id);
    
    let response = Request::post(&url)
        .header("Content-Type", "application/json")
        .header("Api-Key", api_key_ref)
        .body(json_body)?
        .send()
        .await?;

    if response.ok() {
        Ok("Configuration sent to server successfully".to_string())
    } else {
        Err(Error::msg(format!("Error sending configuration to server: {}", response.status_text())))
    }
}

pub async fn add_config_db(db_uri: &str, device_config: &DeviceInfo, api_key: &Option<String>) -> Result<ConfigResponse, Error> {
    let url = format!("{}/api/data/add_config", db_uri);
    let json_body = serde_json::to_string(device_config)?;
    let api_key_ref = api_key.as_deref().ok_or_else(|| anyhow::Error::msg("API key is missing"))?;

    let response = Request::post(&url)
        .header("Content-Type", "application/json")
        .header("Api-Key", api_key_ref)
        .body(json_body)?
        .send()
        .await?;

    if response.ok() {
        let config_response = response.json::<ConfigResponse>().await?;
        Ok(config_response)
    } else {
        Err(Error::msg(format!("Error adding configuration entry to DB: {}", response.status_text())))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveConfigRequest {
    config_id: i32,
}

pub async fn save_config(api_uri: &str, user_id: i32, config_id: i32, api_key: &Option<String>) -> Result<(), Error> {
    let url = format!("{}/api/user/{}/save-config", api_uri, user_id);
    let request_body = SaveConfigRequest { config_id };
    let json_body = serde_json::to_string(&request_body)?;
    let api_key_ref = api_key.as_deref().ok_or_else(|| anyhow::Error::msg("API key is missing"))?;

    let response = Request::post(&url)
        .header("Content-Type", "application/json")
        .header("Api-Key", api_key_ref)
        .body(json_body)?
        .send()
        .await?;

    if response.ok() {
        Ok(())
    } else {
        Err(Error::msg(format!("Error saving configuration: {}", response.status_text())))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SavedConfig {
    pub config_id: i32,
    pub device_hostname: String,
    pub client_name: String,
    pub location: String,
    pub device_type: String,
    pub config_name: String,
    pub storage_location: String,
    pub file_path: String,
    pub saved_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SavedConfigsResponse {
    saved_configs: Vec<SavedConfig>,
}

pub async fn get_saved_configs(api_uri: &str, user_id: i32, api_key: &Option<String>) -> Result<Vec<SavedConfig>, Error> {
    let url = format!("{}/api/user/saved-configs/{}", api_uri, user_id);
    let api_key_ref = api_key.as_deref().ok_or_else(|| anyhow::Error::msg("API key is missing"))?;

    let response = Request::get(&url)
        .header("Api-Key", api_key_ref)
        .send()
        .await?;

    if response.ok() {
        let configs_response = response.json::<SavedConfigsResponse>().await?;
        Ok(configs_response.saved_configs)
    } else {
        Err(Error::msg(format!("Error retrieving saved configurations: {}", response.status_text())))
    }
}

pub async fn get_saved_configs_dummy(api_uri: &str, user_id: i32, api_key: &Option<String>) -> Result<Vec<SavedConfig>, Error> {
    // Simulate a delay to mimic an asynchronous API call
    gloo_timers::future::sleep(Duration::from_millis(500)).await;


    // Dummy data for testing
    let configs = vec![
        SavedConfig {
            config_id: 1,
            device_hostname: "Device1".to_string(),
            client_name: "Alpha Corp".to_string(),
            location: "Location1".to_string(),
            device_type: "Router".to_string(),
            config_name: "Config1".to_string(),
            storage_location: "cloud".to_string(),
            file_path: "/path/to/config1".to_string(),
            saved_at: "2024-01-01 12:00:00".to_string(),
        },
        SavedConfig {
            config_id: 2,
            device_hostname: "Device2".to_string(),
            client_name: "Beta LLC".to_string(),
            location: "Location2".to_string(),
            device_type: "Switch".to_string(),
            config_name: "Config2".to_string(),
            storage_location: "local".to_string(),
            file_path: "/path/to/config2".to_string(),
            saved_at: "2024-01-02 13:00:00".to_string(),
        },
        SavedConfig {
            config_id: 3,
            device_hostname: "Device2".to_string(),
            client_name: "Beta LLC".to_string(),
            location: "Location2".to_string(),
            device_type: "Switch".to_string(),
            config_name: "Config2".to_string(),
            storage_location: "local".to_string(),
            file_path: "/path/to/config2".to_string(),
            saved_at: "2024-01-02 13:00:00".to_string(),
        },
        SavedConfig {
            config_id: 4,
            device_hostname: "Device2".to_string(),
            client_name: "Beta LLC".to_string(),
            location: "Location2".to_string(),
            device_type: "Switch".to_string(),
            config_name: "Config2".to_string(),
            storage_location: "local".to_string(),
            file_path: "/path/to/config2".to_string(),
            saved_at: "2024-01-02 13:00:00".to_string(),
        },
    ];

    Ok(configs)
}
