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
        let package_addr = self.executor.publish_package(package);
        self.packages.insert(String::from(name), package_addr);

        //If first package set as default
        match self.current_package {
            Some(_) => {}
            None => self.current_package = Some(package_addr),
        }

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

        //If first user set as default
        match self.current_user {
            Some(_) => {}
            None => self.current_user = Some(usr),
        }

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

    fn get_current_package(&self) -> Address {
        match self.current_package {
            Some(package) => package,
            None => panic!("Fatal error, no package specified aborting"),
        }
    }

    /// Creates a token returns a ResourceDef
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

    /// Makes a function call and returns a Receipt
    /// # Arguments
    ///
    /// * `package_name`  - The name of the package as named in the blueprint
    /// * `function_name` - The name of the function to call
    /// * `params`        - A vector of Strings with the arguments to pass into the function
    ///
    /// # Examples
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    /// env.create_user("acc1");
    /// env.publish_package(
    ///     "package",
    ///     include_code!("/home/eye/Develop/radixdlt-scrypto/examples/core/gumball-machine/")
    /// );
    /// let receipt = env.call_function("GumballMachine", "new", vec!["0.6".to_owned()]);
    /// assert!(receipt.success);
    /// ```
    pub fn call_function(
        &mut self,
        package_name: &str,
        function_name: &str,
        params: Vec<String>,
    ) -> Receipt {
        let user = self.get_current_user();
        let package = self.get_current_package();
        self.executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_function(
                        package,
                        package_name,
                        function_name,
                        params,
                        Some(user.account),
                    )
                    .deposit_all_buckets(user.account)
                    .build(vec![user.key])
                    .unwrap(),
                false,
            )
            .unwrap()
    }

    /// Makes a method call and returns a Receipt
    /// # Arguments
    ///
    /// * `Component`   - A reference to the Address of the component
    /// * `method_name` - The name of the method
    /// * `params`      - A vector of Strings with the arguments to pass in the method
    ///
    /// # Examples
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    ///
    /// env.create_user("acc1");
    /// env.publish_package(
    ///     "package",
    ///     include_code!("/home/eye/Develop/radixdlt-scrypto/examples/core/gumball-machine/")
    /// );
    ///
    /// let receipt = env.call_function("GumballMachine", "new", vec!["0.6".to_owned()]);
    /// assert!(receipt.success);
    /// let component = receipt.component(0).unwrap();
    ///
    /// let receipt_method = env.call_method(
    ///     &component,
    ///     "buy_gumball",
    ///     vec![format!("1,{}", RADIX_TOKEN)]
    /// );
    /// assert!(receipt_method.success);
    /// ```

    pub fn call_method(
        &mut self,
        component: &Address,
        method_name: &str,
        params: Vec<String>,
    ) -> Receipt {
        let user = self.get_current_user();

        self.executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .call_method(*component, method_name, params, Some(user.account))
                    .deposit_all_buckets(user.account)
                    .build(vec![user.key])
                    .unwrap(),
                false,
            )
            .unwrap()
    }
}
