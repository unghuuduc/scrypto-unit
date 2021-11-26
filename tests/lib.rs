extern crate radix_engine;

use radix_engine::ledger::*;
use scrypto_unit::*;

#[test]
fn test_create_user() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger, &[]);

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
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger, &[]);
    test_env.create_user("alice");
    test_env.get_user("alice");
}

#[test]
fn test_acting_as() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger, &[]);

    let user = test_env.create_user("alice");
    test_env.acting_as("alice");

    assert_eq!(user.account, test_env.current_user.unwrap().account);
    assert_eq!(user.key, test_env.current_user.unwrap().key);
}
