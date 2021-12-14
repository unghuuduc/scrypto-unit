//! # Scrypto Unit
//!
//! `scrypto_unit` is a lightweight testing framework for Scrypto.
//!
//! This crate contains a collection of useful methods that you can
//! leverage when testing your components.
#![allow(dead_code)]

extern crate radix_engine;
extern crate scrypto;

use radix_engine::ledger::*;
use radix_engine::model::Auth::NoAuth;
use radix_engine::transaction::*;
use radix_engine::utils::*;
use scrypto::prelude::*;
use sbor::Decode;

#[derive(Debug, Copy, Clone, PartialEq)]
/// The user account.
pub struct User {
    /// The user's public key.
    pub key: Address,
    /// The user's account address.
    pub account: Address,
}
/// Represents a test environment.
pub struct TestEnv<'a, L: Ledger> {
    /// The transaction executioner.
    pub executor: TransactionExecutor<'a, L>,
    /// The users of the test environment.
    pub users: HashMap<String, User>,
    /// The current user of the test environment.
    pub current_user: Option<User>,
    /// The test environment packages.
    pub packages: HashMap<String, Address>,
    /// The current package of the test environment.
    pub current_package: Option<Address>,
}

impl<'a, L: Ledger> TestEnv<'a, L> {
    /// Returns a test environment instance with the following fields:
    ///
    /// * `executor` - The transaction executioner.
    /// * `users` - The users of the test environment.
    /// * `current_user` - The current user of the test environment.
    /// * `packages` - The test environment packages.
    /// * `current_package` - The current package of the test environment.
    ///
    /// # Arguments
    ///
    /// * `ledger` - The transaction execution ledger.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    /// ```
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

    /// Publishes a given package to the transaction execution ledger.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the package.
    /// * `package` - The package as a binary array.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    ///
    /// env.publish_package(
    ///     "package",
    ///     include_code!("../../radixdlt-scrypto/examples/core/gumball-machine/")
    /// );
    /// ```
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

    /// Retrieve a test environment package by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the package.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    ///
    /// env.publish_package(
    ///     "package",
    ///     include_code!("../../radixdlt-scrypto/examples/core/gumball-machine/")
    /// );
    ///
    /// let package = env.get_package("package");
    /// ```
    pub fn get_package(&self, name: &str) -> Address {
        match self.packages.get(name) {
            Some(&package) => package,
            None => panic!("No package named {:?} found.", name),
        }
    }

    /// Sets the current package of the test environment.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the package.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    ///
    /// env.publish_package(
    ///     "package",
    ///     include_code!("../../radixdlt-scrypto/examples/core/gumball-machine/")
    /// );
    ///
    /// env.using_package("package");
    /// ```
    pub fn using_package(&mut self, name: &str) -> &mut Self {
        let package = self.get_package(name);
        self.current_package = Some(package);

        self
    }

    /// Create a test user.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the user.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    ///
    /// env.create_user("test user");
    /// ```
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

    /// Retrieve a test user by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the user.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    /// 
    /// env.create_user("test user");
    ///
    /// let user = env.get_user("test user");
    /// ```
    pub fn get_user(&self, name: &str) -> &User {
        match self.users.get(name) {
            Some(user) => user,
            None => panic!("No user named {:?} found.", name),
        }
    }

    /// Set the current user of the test environment.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the user.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    ///
    /// env.create_user("test user");
    ///
    /// env.acting_as("test user");
    /// 
    /// assert_eq!(env.get_current_user(), *env.get_user("test user"))
    /// ```
    pub fn acting_as(&mut self, name: &str) -> &mut Self {
        let user = self.get_user(name);
        self.current_user = Some(*user);

        self
    }

    /// Returns the current test user.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    /// 
    /// let user = env.create_user("acc1");
    /// 
    /// let current_user = env.get_current_user();
    /// 
    /// assert_eq!(user, current_user);
    /// ```
    pub fn get_current_user(&self) -> User {
        match self.current_user {
            Some(user) => user,
            None => panic!("Fatal error, no user specified aborting"),
        }
    }

    /// Returns the current test package.
    ///
    /// # Examples
    ///
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::*;
    /// use scrypto::prelude::*;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    ///
    /// env.publish_package(
    ///     "package",
    ///     include_code!("../../radixdlt-scrypto/examples/core/gumball-machine/")
    /// );
    ///
    /// let current_package = env.get_current_package();
    /// ```
    pub fn get_current_package(&self) -> Address {
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
                    .drop_all_bucket_refs()
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
    ///     include_code!("../../radixdlt-scrypto/examples/core/gumball-machine/")
    /// );
    /// let receipt = env.call_function("GumballMachine", "new", vec!["0.6".to_owned()]);
    /// assert!(receipt.success);
    /// ```
    pub fn call_function(
        &mut self,
        blueprint_name: &str,
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
                        blueprint_name,
                        function_name,
                        params,
                        Some(user.account),
                    )
                    .drop_all_bucket_refs()
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
    ///     include_code!("../../radixdlt-scrypto/examples/core/gumball-machine/")
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
                    .drop_all_bucket_refs()
                    .deposit_all_buckets(user.account)
                    .build(vec![user.key])
                    .unwrap(),
                false,
            )
            .unwrap()
    }

    fn get_vault_info(ledger: &L, vid: Vid) -> (Address, Decimal) {
        let vault = ledger.get_vault(vid).unwrap();
        let amount = vault.amount(NoAuth).unwrap();
        let resource_def_address = vault.resource_def(NoAuth).unwrap();

        (resource_def_address, amount)
    }

    /// Returns the amount of the resource for the component/account
    /// # Arguments
    ///
    /// * `component`    - The Address of the component that holds the resource
    /// * `resource_def` - The Address that holds the resource
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
    /// let user = env.create_user("acc1");
    /// let amount = env.get_amount_for_rd(user.account, RADIX_TOKEN);  
    /// assert!( amount == 1000000.into() );
    /// ```
    pub fn get_amount_for_rd(&mut self, component: Address, resource_def: Address) -> Decimal {
        let ledger = self.executor.ledger();
        let mut vids: Vec<Vid> = Vec::new();
        let component = ledger.get_component(component).unwrap();
        let state = component.state(NoAuth).unwrap();
        format_data_with_ledger(&state, ledger, &mut vids).unwrap();

        for vid in vids {
            let (resource_def_address, amount) = TestEnv::get_vault_info(ledger, vid);
            if resource_def_address == resource_def {
                return amount;
            }
        }

        0.into()
    }

    /// Returns a HashMap with the addresses of the vaults and their amounts. Works with a component or account.
    /// # Arguments
    ///
    /// * `ledger`   - A reference to the InMemoryLedger
    /// * `address`  - The Address of the component
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
    /// let user = env.create_user("acc1");
    /// let vaults = env.get_account_vaults(user.account);  
    ///
    /// for (addr, amt) in vaults {
    ///     println!("Address: {}, Amount: {}", addr, amt);
    ///     if addr == RADIX_TOKEN {
    ///         assert!(amt == 1000000.into());
    ///     }
    /// }
    /// ```
    pub fn get_account_vaults(&mut self, address: Address) -> HashMap<Address, Decimal> {
        let mut vids: Vec<Vid> = Vec::new();
        let ledger = self.executor.ledger();
        let component = ledger.get_component(address).unwrap();
        let state = component.state(NoAuth).unwrap();
        format_data_with_ledger(&state, ledger, &mut vids).unwrap();
        vids.drain(..)
            .map(|vid| TestEnv::get_vault_info(ledger, vid))
            .collect()
    }

    /// Transfers some resource between users
    /// # Arguments
    ///
    /// * `amount` - A decimal that defines the amount to transfer
    /// * `resource_def` - The resource_def for the resource to transfer
    /// * `to_user` - the user receiving the amount of resource
    ///
    /// # Examples
    /// ```
    /// use scrypto_unit::*;
    /// use radix_engine::ledger::InMemoryLedger;
    ///
    /// let mut ledger = InMemoryLedger::with_bootstrap();
    /// let mut env = TestEnv::new(&mut ledger);
    /// env.create_user("user1");
    /// let token = env.create_token(10000.into());
    /// let user2 = env.create_user("user2");
    /// env.transfer_resource(10.into(), &token, &user2);
    /// ```
    pub fn transfer_resource(&mut self, amount: Decimal, resource_def: &ResourceDef, to_user: &User) -> Receipt {
        let user = self.get_current_user();
        let receipt = self
            .executor
            .run(
                TransactionBuilder::new(&self.executor)
                    .withdraw_from_account(amount, resource_def.address(), user.account)
                    .drop_all_bucket_refs()
                    .deposit_all_buckets(to_user.account)
                    .build(vec![user.key])
                    .unwrap(),
                false,
            )
            .unwrap();

        receipt
    }

}

/// Decodes the return value from a blueprint function within a transaction from the receipt
/// # Arguments
///
/// * `receipt`  - The name of the package as named in the blueprint
/// * `blueprint_name` - The name of the blueprint to search for the matching Instruction::CallFunction
/// 
/// NOTE: a custom built transaction may have more than one matching call.  This convenience
///       function may not work in such cases.
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
/// env.publish_package(
///     "package",
///     include_code!("../../radixdlt-scrypto/examples/core/gumball-machine/")
/// );
/// 
/// env.create_user("test user");
/// env.acting_as("test user");
/// 
/// const BLUEPRINT: &str = "GumballMachine";
/// let mut receipt = env.call_function(BLUEPRINT, "new", vec!["0.6".to_owned()]);
/// assert!(receipt.success);
/// let ret: Component = return_of_call_function(&mut receipt, BLUEPRINT);
/// ```
pub fn return_of_call_function<T: Decode>(receipt: &mut Receipt, blueprint_name: &str) -> T {
    let instruction_index = receipt.transaction.instructions.iter().position(|i| match i { Instruction::CallFunction { ref blueprint, .. } if blueprint == blueprint_name => true, _ => false }).unwrap();
    let encoded = receipt.results.swap_remove(instruction_index).unwrap().unwrap().encoded;
    scrypto_decode(&encoded).unwrap()
}
