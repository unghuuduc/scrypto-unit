extern crate radix_engine;

use radix_engine::ledger::*;
use scrypto_unit::*;

#[test]
fn test_create_user() {
    let mut ledger = InMemoryLedger::with_bootstrap();
    let mut test_env = TestEnv::new(&mut ledger);

    test_env.create_account("alice");
    test_env.create_account("bob");
    test_env.create_account("carol");

    assert!(test_env.users.contains_key("alice"));
    assert!(test_env.users.contains_key("bob"));
    assert!(test_env.users.contains_key("carol"));
}
