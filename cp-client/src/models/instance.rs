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
pub struct Instance {
    // #[serde(
    //     rename = "app_services",
    //     default,
    //     with = "::serde_with::rust::double_option",
    //     skip_serializing_if = "Option::is_none"
    // )]
    // pub app_services: Option<Option<Vec<models::AppType>>>,
    #[serde(rename = "autoscaling")]
    pub autoscaling: Box<models::Autoscaling>,
    #[serde(
        rename = "connection_info",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_info: Option<Option<Box<models::ConnectionInfo>>>,
    #[serde(
        rename = "connection_pooler",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub connection_pooler: Option<Option<Box<models::ConnectionPooler>>>,
    #[serde(rename = "cpu")]
    pub cpu: models::Cpu,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "dataplane_index")]
    pub dataplane_index: String,
    #[serde(
        rename = "dedicated_networking",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub dedicated_networking: Option<Option<Box<models::DedicatedNetworking>>>,
    #[serde(rename = "environment")]
    pub environment: models::Environment,
    #[serde(
        rename = "extensions",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub extensions: Option<Option<Vec<models::ExtensionStatus>>>,
    #[serde(
        rename = "extra_domains_rw",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub extra_domains_rw: Option<Option<Vec<String>>>,
    #[serde(
        rename = "first_recoverability_time",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub first_recoverability_time: Option<Option<String>>,
    #[serde(
        rename = "image",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub image: Option<Option<String>>,
    #[serde(rename = "instance_id")]
    pub instance_id: String,
    #[serde(rename = "instance_name")]
    pub instance_name: String,
    #[serde(
        rename = "ip_allow_list",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub ip_allow_list: Option<Option<Vec<String>>>,
    #[serde(rename = "last_updated_at", skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<String>,
    #[serde(
        rename = "last_wal_archive_status",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_wal_archive_status: Option<Option<String>>,
    #[serde(rename = "memory")]
    pub memory: models::Memory,
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "organization_id")]
    pub organization_id: String,
    #[serde(rename = "organization_name")]
    pub organization_name: String,
    #[serde(
        rename = "postgres_configs",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub postgres_configs: Option<Option<Vec<models::PgConfig>>>,
    /// Major Postgres version this instance is using. Currently: 14, 15, 16 and 17
    #[serde(rename = "postgres_version")]
    pub postgres_version: i32,
    #[serde(rename = "provider_id")]
    pub provider_id: String,
    #[serde(rename = "region_id")]
    pub region_id: String,
    #[serde(rename = "region_name")]
    pub region_name: String,
    #[serde(rename = "replicas")]
    pub replicas: i32,
    #[serde(
        rename = "runtime_config",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub runtime_config: Option<Option<Vec<models::PgConfig>>>,
    #[serde(
        rename = "spot",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub spot: Option<Option<bool>>,
    #[serde(rename = "stack_type")]
    pub stack_type: models::StackType,
    #[serde(rename = "state")]
    pub state: models::State,
    #[serde(rename = "storage")]
    pub storage: models::Storage,
    #[serde(
        rename = "trunk_installs",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub trunk_installs: Option<Option<Vec<models::TrunkInstallStatus>>>,
}

impl Instance {
    pub fn new(
        autoscaling: models::Autoscaling,
        cpu: models::Cpu,
        dataplane_index: String,
        environment: models::Environment,
        instance_id: String,
        instance_name: String,
        memory: models::Memory,
        namespace: String,
        organization_id: String,
        organization_name: String,
        postgres_version: i32,
        provider_id: String,
        region_id: String,
        region_name: String,
        replicas: i32,
        stack_type: models::StackType,
        state: models::State,
        storage: models::Storage,
    ) -> Instance {
        Instance {
            autoscaling: Box::new(autoscaling),
            connection_info: None,
            connection_pooler: None,
            cpu,
            created_at: None,
            dataplane_index,
            dedicated_networking: None,
            environment,
            extensions: None,
            extra_domains_rw: None,
            first_recoverability_time: None,
            image: None,
            instance_id,
            instance_name,
            ip_allow_list: None,
            last_updated_at: None,
            last_wal_archive_status: None,
            memory,
            namespace,
            organization_id,
            organization_name,
            postgres_configs: None,
            postgres_version,
            provider_id,
            region_id,
            region_name,
            replicas,
            runtime_config: None,
            spot: None,
            stack_type,
            state,
            storage,
            trunk_installs: None,
        }
    }
}
