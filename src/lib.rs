#[allow(unused_imports)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, serde_json, Promise, env};
use near_sdk::serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::iter::Sum;
pub type AccountId = String;

mod data;

#[derive(BorshDeserialize,BorshSerialize,Serialize,Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Msg {
    recipient: String,
    msg: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Packages {
    sub_package: String,
    channels: Vec<String>,
    price: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Subscription {
    package_id: String,
    user: String,
}

#[near_bindgen]
pub struct Media {
    packages:HashMap<String, Packages>,
    messages:Vec<Msg>,
    subscriptions:Vec<Subscription>,   
}

#[near_bindgen]
impl Media {
    #[init]
    pub fn default_package() -> Self {
        let mut packages:HashMap<String, Packages> = HashMap::new();
        let messages:Vec<Msg> = Vec::new();
        let subscriptions:Vec<Subscription> = Vec::new();

        Media { 
            packages, 
            messages, 
            subscriptions,
        }
    }
    
    pub fn subscribe(&mut self, package_id:String, user:String) {
        match self.packages.get(&package_id) {
            Some(value) => {
                let price = value.price*1_000_000_000_000_000_000_000_000;

                Promise::new(env::current_account_id()).transfer(price);

                self.messages.push(Msg { recipient:user.to_string(), msg: "Subscription successful".to_string()});
                let subscriptions = &mut self.subscriptions;

                let mut counter = 0;
                subscriptions.into_iter().for_each(|subscription| {
                    if subscription.user == user {
                        counter+=1;
                        subscription.package_id = package_id.to_string();
                    }
                });
                if counter <1 {
                    self.subscriptions.push(Subscription {package_id:package_id.to_string(), user:user.to_string()});
                    self.messages.push(Msg { recipient:user.to_string(), msg: "Subscription successful".to_string()});
                }
            }
            None => {
                env::log_str("Invalid package_id");
            }
        }
    }
    pub fn count_subscription(&self) -> usize {
        return self.subscriptions.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};

    fn get_context (current:AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.signer_account_id(current);
        builder.account_balance(1_000_000_000_000_000_000_000_000_000);
        return builder;
    }

    #[test]
    fn subscription (){
        let ted = AccountId::new_unchecked("tedadams.testnet".to_string());
        let context = get_context(ted.clone());
        testing_env!(context.build());
        let mut contract = Media::default_package();
        contract.subscribe("Nyota".to_string(), "tedadams.testnet".to_string());
        assert_eq!(contract.count_subscription(),1);
    }
}
    