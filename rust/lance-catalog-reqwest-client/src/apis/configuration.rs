/*
 * Lance Catalog REST Specification
 *
 * **Lance Catalog** is an OpenAPI specification on top of the storage-based Lance format. It provides an integration point for catalog service like Apache Hive MetaStore (HMS), Apache Gravitino, etc. to store and use Lance tables. To integrate, the catalog service implements a **Lance Catalog Adapter**, which is a REST server that converts the Lance catalog requests to native requests against the catalog service. Different tools can integrate with Lance Catalog using the generated OpenAPI clients in various languages, and invoke operations in Lance Catalog to read, write and manage Lance tables in the integrated catalog services. 
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */



#[derive(Debug, Clone)]
pub struct Configuration {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: reqwest::Client,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub bearer_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}


impl Configuration {
    pub fn new() -> Configuration {
        Configuration::default()
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            base_path: "http://localhost:2333".to_owned(),
            user_agent: Some("OpenAPI-Generator/0.0.1/rust".to_owned()),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: None,
            api_key: None,
        }
    }
}
