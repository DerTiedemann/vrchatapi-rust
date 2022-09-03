/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ariesclark.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`get_instance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstanceError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_instance_by_short_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetInstanceByShortNameError {
    Status401(crate::models::Error),
    Status404(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_short_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShortNameError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`send_self_invite`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendSelfInviteError {
    Status401(crate::models::Error),
    UnknownValue(serde_json::Value),
}


/// Returns an instance. Please read [Instances Tutorial](https://vrchatapi.github.io/tutorials/instances/) for more information on Instances.  If an invalid instanceId is provided, this endpoint will simply return \"null\"!
pub fn get_instance(configuration: &configuration::Configuration, world_id: &str, instance_id: &str) -> Result<crate::models::Instance, Error<GetInstanceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/instances/{worldId}:{instanceId}", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id), instanceId=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetInstanceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an instance. Please read [Instances Tutorial](https://vrchatapi.github.io/tutorials/instances/) for more information on Instances.
pub fn get_instance_by_short_name(configuration: &configuration::Configuration, short_name: &str) -> Result<crate::models::Instance, Error<GetInstanceByShortNameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/instances/s/{shortName}", local_var_configuration.base_path, shortName=crate::apis::urlencode(short_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetInstanceByShortNameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an instance short name.
pub fn get_short_name(configuration: &configuration::Configuration, world_id: &str, instance_id: &str) -> Result<String, Error<GetShortNameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/instances/{worldId}:{instanceId}/shortName", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id), instanceId=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetShortNameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sends an invite to the instance to yourself.
pub fn send_self_invite(configuration: &configuration::Configuration, world_id: &str, instance_id: &str) -> Result<crate::models::Success, Error<SendSelfInviteError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/instances/{worldId}:{instanceId}/invite", local_var_configuration.base_path, worldId=crate::apis::urlencode(world_id), instanceId=crate::apis::urlencode(instance_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let mut local_var_resp = local_var_client.execute(local_var_req)?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text()?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SendSelfInviteError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

