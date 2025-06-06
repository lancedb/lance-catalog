/*
 * Lance REST Namespace Specification
 *
 * **Lance Namespace Specification** is an open specification on top of the storage-based Lance data format  to standardize access to a collection of Lance tables (a.k.a. Lance datasets). It describes how a metadata service like Apache Hive MetaStore (HMS), Apache Gravitino, Unity Catalog, etc. should store and use Lance tables, as well as how ML/AI tools and analytics compute engines (will together be called _\"tools\"_ in this document) should integrate with Lance tables. A Lance namespace is a centralized repository for discovering, organizing, and managing Lance tables. It can either contain a collection of tables, or a collection of Lance namespaces recursively. It is designed to encapsulates concepts including namespace, metastore, database, schema, etc. that frequently appear in other similar data systems to allow easy integration with any system of any type of object hierarchy. In an enterprise environment, typically there is a requirement to store tables in a metadata service  for more advanced governance features around access control, auditing, lineage tracking, etc. **Lance REST Namespace** is an OpenAPI protocol that enables reading, writing and managing Lance tables by connecting those metadata services or building a custom metadata server in a standardized way. 
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`alter_transaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AlterTransactionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status409(models::ErrorResponse),
    Status503(models::ErrorResponse),
    Status5XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_transaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTransactionError {
    Status400(models::ErrorResponse),
    Status401(models::ErrorResponse),
    Status403(models::ErrorResponse),
    Status404(models::ErrorResponse),
    Status503(models::ErrorResponse),
    Status5XX(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn alter_transaction(configuration: &configuration::Configuration, alter_transaction_request: models::AlterTransactionRequest) -> Result<models::AlterTransactionResponse, Error<AlterTransactionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_alter_transaction_request = alter_transaction_request;

    let uri_str = format!("{}/AlterTransaction", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_alter_transaction_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::AlterTransactionResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::AlterTransactionResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AlterTransactionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Return a detailed information for a given transaction
pub async fn get_transaction(configuration: &configuration::Configuration, get_transaction_request: models::GetTransactionRequest) -> Result<models::GetTransactionResponse, Error<GetTransactionError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_get_transaction_request = get_transaction_request;

    let uri_str = format!("{}/GetTransaction", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    req_builder = req_builder.json(&p_get_transaction_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetTransactionResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetTransactionResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetTransactionError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

