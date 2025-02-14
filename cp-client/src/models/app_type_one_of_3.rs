/*
 * Tembo Cloud
 *
 * Platform API for Tembo Cloud             </br>             </br>             To find a Tembo Data API, please find it here:             </br>             </br>             [AWS US East 1](https://api.data-1.use1.tembo.io/swagger-ui/)
 *
 * The version of the OpenAPI document: v1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppTypeOneOf3 {
    #[serde(rename = "embeddings", deserialize_with = "Option::deserialize")]
    pub embeddings: Option<Box<models::AppConfig>>,
}

impl AppTypeOneOf3 {
    pub fn new(embeddings: Option<models::AppConfig>) -> AppTypeOneOf3 {
        AppTypeOneOf3 {
            embeddings: if let Some(x) = embeddings {
                Some(Box::new(x))
            } else {
                None
            },
        }
    }
}
