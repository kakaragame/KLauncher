use crate::utils;
use std::fs;
use serde::{Serialize, Deserialize, Serializer};
use crate::error::KError;
use crate::error::Level;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Account {
    pub first_name: String,
    pub email: String,
    pub game_accounts: Vec<GameAccount>,
}


impl Account {}

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct GameAccount {
    pub uuid: String,
    pub login_token: String,
}


impl GameAccount {}