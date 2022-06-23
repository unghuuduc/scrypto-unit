extern crate radix_engine;

use radix_engine::ledger::*;
use scrypto::prelude::*;
use scrypto_unit::*;

const PACKAGE: &str = "hello-world";
const BLUEPRINT: &str = "Hello";

#[test]
fn test_create_user() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    test_env.create_user("alice");
    test_env.create_user("bob");
    test_env.create_user("carol");

    assert!(test_env.users.contains_key("alice"));
    assert!(test_env.users.contains_key("bob"));
    assert!(test_env.users.contains_key("carol"));
    assert_eq!(test_env.users.len(), 3);
}

#[test]
fn test_get_user() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);
    test_env.create_user("alice");
    test_env.get_user("alice");
}

#[test]
fn test_acting_as() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    let user = test_env.create_user("alice");
    test_env.acting_as("alice");

    assert_eq!(user.account, test_env.current_user.unwrap().account);
    assert_eq!(user.key, test_env.current_user.unwrap().key);
}

#[test]
fn test_build_package() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    test_env.create_user("admin");
    test_env.create_user("user");

    test_env.acting_as("admin");
    let package = compile_package!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/hello-world/",
    ));

    test_env.publish_package(PACKAGE, &package);
    test_env.using_package(PACKAGE);
    test_env.get_package(PACKAGE);
}

#[test]
fn test_package_functions() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    test_env.create_user("admin");
    test_env.create_user("user");

    test_env.acting_as("admin");
    let package = compile_package!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/hello-world/",
    ));

    test_env.publish_package(PACKAGE, &package);
    // no need to call using_package. Just published package is set as default.

    let instantiate_receipt = test_env.call_function(BLUEPRINT, "instantiate", vec![]);
    assert!(instantiate_receipt.result.is_ok());  
}

#[test]
/// Calling blueprint function that does not exist 
fn test_package_functions_error() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    test_env.create_user("admin");
    test_env.create_user("user");

    test_env.acting_as("admin");
    let package = compile_package!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/hello-world/",
    ));

    test_env.publish_package(PACKAGE, &package);
    // no need to call using_package. Just published package is set as default.

    let instantiate_receipt = test_env.call_function(BLUEPRINT, "instantiate_other", vec![]);
    assert!(instantiate_receipt.result.is_err());  
}

#[test]
fn test_component_func_auth() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    test_env.create_user("admin");
    test_env.create_user("user");

    test_env.acting_as("admin");
    let package = compile_package!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/hello-world/",
    ));

    test_env.publish_package(PACKAGE, &package);
    // no need to call using_package. Just published package is set as default.

    let instantiate_receipt = test_env.call_function(BLUEPRINT, "instantiate", vec![]);
    assert!(instantiate_receipt.result.is_ok());

    let hello_component = instantiate_receipt.new_component_addresses[0];
    let admin_badge = instantiate_receipt.new_resource_addresses[0];
    
    let auth_method_receipt = test_env.call_method_auth(hello_component, "update_state", admin_badge, vec![scrypto_encode(&42u32)]);
    println!("{:?}", auth_method_receipt);
    assert!(auth_method_receipt.result.is_ok());
}

#[test]
fn test_component_func() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    test_env.create_user("admin");
    test_env.create_user("user");

    test_env.acting_as("admin");
    let package = compile_package!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/assets/hello-world/",
    ));

    test_env.publish_package(PACKAGE, &package);
    // no need to call using_package. Just published package is set as default.

    let instantiate_receipt = test_env.call_function(BLUEPRINT, "instantiate", vec![]);
    assert!(instantiate_receipt.result.is_ok());

    let hello_component = instantiate_receipt.new_component_addresses[0];
    let _admin_badge = instantiate_receipt.new_resource_addresses[0];
    
    let method_receipt = test_env.call_method(hello_component, "update_state", vec![scrypto_encode(&42u32)]);
    println!("{:?}", method_receipt);
    assert!(method_receipt.result.is_ok());
}

#[test]
fn test_create_token_send_amount() {
    let mut ledger = InMemorySubstateStore::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    let _admin = test_env.create_user("admin");
    let user = test_env.create_user("user");

    test_env.acting_as("admin");

    //creating token and seding it from admin (cause we act as admin) to user
    let token = test_env.create_token(dec!("10000"));
    let transfer_receipt = test_env.transfer_resource(dec!("10"), &token, &user);
    println!("{:?}", transfer_receipt);
    assert!(transfer_receipt.result.is_ok());
    //TODO: assert balance of user before->after using test_env.get_amount_for_rd()
}

