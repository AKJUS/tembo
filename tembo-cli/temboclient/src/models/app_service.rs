/*
 * Tembo Cloud
 *
 * Platform API for Tembo Cloud             </br>             </br>             To find a Tembo Data API, please find it here:             </br>             </br>             [AWS US East 1](https://api.data-1.use1.tembo.io/swagger-ui/)             
 *
 * The version of the OpenAPI document: v1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppService {
    #[serde(rename = "args", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub args: Option<Option<Vec<String>>>,
    #[serde(rename = "command", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub command: Option<Option<Vec<String>>>,
    #[serde(rename = "env", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub env: Option<Option<Vec<crate::models::EnvVar>>>,
    #[serde(rename = "image")]
    pub image: String,
    #[serde(rename = "middlewares", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub middlewares: Option<Option<Vec<crate::models::Middleware>>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "probes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub probes: Option<Option<Box<crate::models::Probes>>>,
    #[serde(rename = "resources", skip_serializing_if = "Option::is_none")]
    pub resources: Option<Box<crate::models::ResourceRequirements>>,
    #[serde(rename = "routing", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub routing: Option<Option<Vec<crate::models::Routing>>>,
    #[serde(rename = "storage", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub storage: Option<Option<Box<crate::models::StorageConfig>>>,
}

impl AppService {
    pub fn new(image: String, name: String) -> AppService {
        AppService {
            args: None,
            command: None,
            env: None,
            image,
            middlewares: None,
            name,
            probes: None,
            resources: None,
            routing: None,
            storage: None,
        }
    }
}

