use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshow::REQWEST_CLIENT;
use crate::model:notification::Notification;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate= "rocket::serder")]
pub struct: Subscriber{
    pub url: String,
    pub name: String,
}