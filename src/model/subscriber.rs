use rocket::serde::{Deserialize, Serialize};
use rocket::log;
use rocket::serde::json::to_string;
use rocket::tokio;
use bambangshow::REQWEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub name: String,
    pub url: String,
}