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
    pub current_package: Option<Address>,
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
            packages,
            current_package: None,
        }
    }

    pub fn publish_package(&mut self, name: &str, package: &[u8]) -> &mut Self {
        let package = self.executor.publish_package(package);
        self.packages.insert(String::from(name), package);

        self
    }

    pub fn get_package(&self, name: &str) -> Address {
        match self.packages.get(name) {
            Some(&package) => package,
            None => panic!("No package named {:?} found.", name),
        }
    }

    pub fn using_package(&mut self, name: &str) -> &mut Self {
        let package = self.get_package(name);
        self.current_package = Some(package);

        self
    }

    pub fn create_user(&mut self, name: &str) -> User {
        let key = self.executor.new_public_key();
        let account = self.executor.new_account(key);

        self.users.insert(String::from(name), User { key, account });

        let usr = User { key, account };

        //Set user as default user
        self.current_user = Some(usr);
        usr
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

    fn get_current_user(&self) -> User {
        match self.current_user {
            Some(user) => user,
            None => panic!("Fatal error, no user specified aborting"),
        }
    }

    ///Creates a token returns a ResourceDef
    /// # Arguments
    ///
    /// * `max_supply` - A decimal that defines the supply
    ///
    /// # Examples
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::InMemoryLedger;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    /// env.create_user("acc1");
    /// let token = env.create_token(10000.into());
    /// ```
    pub fn create_token(&mut self, max_supply: Decimal) -> ResourceDef {
        let user = self.get_current_user();
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .new_token_fixed(HashMap::new(), max_supply.into())
                    .deposit_all_buckets(user.account)
                    .build(vec![user.key])
                    .unwrap(),
                false,
            )
            .unwrap();

        return receipt.resource_def(0).unwrap().into();
    }
}
