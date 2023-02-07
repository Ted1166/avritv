extern crate reqwest;
extern crate serde;
extern crate json;

// use chrono::prelude::*;
use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ near_bindgen, Promise, env };
use near_sdk::serde::{ Serialize, Deserialize };
use std::collections::HashMap;
// use std::fmt::format;
use std::fs;
// use serde_json::Value;
use std::string::String;
pub type AccountId = String;

pub mod config;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Msg {
    recipient: String,
    msg: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Packages {
    name: String,
    channels: Vec<String>,
    price: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Subscription {
    package_id: String,
    user: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
pub struct Channel {
    name: String,
    urls: Vec<String>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
pub struct Config {
    packages: Vec<Packages>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Debug)]
#[near_bindgen]
pub struct Media {
    packages: HashMap<String, Packages>,
    messages: Vec<Msg>,
    subscriptions: Vec<Subscription>,
}


impl Media {
    pub fn new() -> Self {
        let packages: HashMap<String, Packages> = HashMap::new();
        let messages: Vec<Msg> = Vec::new();
        let subscriptions: Vec<Subscription> = Vec::new();

        Media { packages, messages, subscriptions }
    }

    pub fn import_json(&mut self, file_path: String) -> String {
        let file = fs::read_to_string(file_path).unwrap();
        let media: Media = serde_json::from_str(&file).unwrap();
        format!(
            "media packages: {:?} media messages: {:?} media subscriptions: {:?}",
            media.packages,
            media.messages,
            media.subscriptions
        )
    }

    pub fn set_price(&mut self, package_name: String, price: u128) -> String {
        for package in &mut self.packages {
            // let packages: HashMap<String, Packages> = HashMap::new();
            if package.1.name == package_name {
                package.1.price = price;
                return format!("package: {} price changed to {}", package_name, price);
            }
        }
        format!("package: {} not found", package_name)
    }

    pub fn subscribe(&mut self, package_id: String, user: String) {
        match self.packages.get(&package_id) {
            Some(value) => {
                let price = value.price * 1_000_000_000_000_000_000_000_000;

                Promise::new(env::current_account_id()).transfer(price);

                self.messages.push(Msg {
                    recipient: user.to_string(),
                    msg: "Subscription successful".to_string(),
                });
                let subscriptions = &mut self.subscriptions;

                let mut counter = 0;
                subscriptions.iter_mut().for_each(|subscription| {
                    if subscription.user == user {
                        counter += 1;
                        subscription.package_id = package_id.to_string();
                    }
                });
                if counter < 1 {
                    self.subscriptions.push(Subscription {
                        package_id: package_id.to_string(),
                        user: user.to_string(),
                    });
                    self.messages.push(Msg {
                        recipient: user,
                        msg: "Subscription successful".to_string(),
                    });
                }
            }
            None => {
                env::log_str("Invalid package_id");
            }
        }
    }

    pub fn count_subscription(&mut self) -> usize {
        self.subscriptions.len()
    }

    pub fn count_channels(&mut self) -> usize {
        self.subscriptions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{ testing_env, AccountId };

    fn get_context(current: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(current);
        builder.account_balance(1_000_000_000_000_000_000_000_000_000);
        return builder;
    }

    #[test]
    fn subscription() {
        let ted = AccountId::new_unchecked("tedadams.testnet".to_string());
        let context = get_context(ted.clone());
        testing_env!(context.build());
        let file_path = "config.rs".to_string();
        let mut contract = Media::new();
        contract.import_json(file_path);
        contract.subscribe("Nyota".to_string(), "tedadams.testnet".to_string());
        assert_eq!(contract.count_subscription(), 1);
    }

    #[test]
    fn channels() {
        let ted = AccountId::new_unchecked("tedadams.testnet".to_string());
        let context = get_context(ted.clone());
        testing_env!(context.build());
        let mut contract = Media::new();
        contract.subscribe("Nyota".to_string(), "tedadams.testnet".to_string());
        assert_eq!(contract.count_channels(), 0);
    }
}