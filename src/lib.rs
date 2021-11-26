#![allow(dead_code)]

extern crate radix_engine;
extern crate scrypto;

use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct User {
    pub key: Address,
    pub account: Address,
}

pub struct TestEnv<'a, L: Ledger> {
    pub executor: TransactionExecutor<'a, L>,
    pub users: HashMap<String, User>,
    pub current_user: Option<User>,
    pub packages: HashMap<String, Address>,
}

impl<'a, L: Ledger> TestEnv<'a, L> {
    pub fn new(ledger: &'a mut L) -> Self {
        let executor = TransactionExecutor::new(ledger, 0, 0);
        let users: HashMap<String, User> = HashMap::new();
        let packages: HashMap<String, Address> = HashMap::new();

        Self {
            executor,
            users,
            current_user: None,
            packages
        }
    }

    pub fn publish_package(&mut self, name: &str, package: &[u8]) -> &mut Self {
        let package = self.executor.publish_package(package);
        self.packages.insert(String::from(name), package);

        self
    }

    pub fn create_user(&mut self, name: &str) -> User {
        let key = self.executor.new_public_key();
        let account = self.executor.new_account(key);

        self.users.insert(String::from(name), User { key, account });

        User { key, account }
    }

    pub fn get_user(&self, name: &str) -> &User {
        match self.users.get(name) {
            Some(user) => user,
            None => panic!("No user named {:?} found.", name),
        }
    }

    pub fn acting_as(&mut self, name: &str) -> &mut Self {
        let user = self.get_user(name);
        self.current_user = Some(*user);

        self
    }
}
