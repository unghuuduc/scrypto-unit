#![allow(dead_code)]

extern crate radix_engine;
extern crate scrypto;

use radix_engine::ledger::*;
use radix_engine::transaction::*;
use scrypto::prelude::*;

#[derive(Debug)]
pub struct User {
    pub key: Address,
    pub account: Address,
}

pub struct TestEnv<'a, L: Ledger> {
    pub executor: TransactionExecutor<'a, L>,
    pub users: HashMap<String, User>,
}

impl<'a, L: Ledger> TestEnv<'a, L> {
    pub fn new(ledger: &'a mut L) -> Self {
        let executor = TransactionExecutor::new(ledger, 0, 0);
        let users: HashMap<String, User> = HashMap::new();

        Self { executor, users }
    }

    pub fn create_account(&mut self, name: &str) -> &mut Self {
        let key = self.executor.new_public_key();
        let account = self.executor.new_account(key);
        let user = User { key, account };

        self.users.insert(String::from(name), user);

        self
    }
}
