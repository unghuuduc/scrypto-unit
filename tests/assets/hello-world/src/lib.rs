use scrypto::prelude::*;

blueprint! {
    struct Hello {
        state: u32,
        /// A badge used to administer the component
        admin_badge: ResourceAddress,
    }

    impl Hello {
        pub fn instantiate() -> (ComponentAddress, Bucket) {
            let admin_badge = ResourceBuilder::new_fungible()
                .divisibility(DIVISIBILITY_NONE)
                .metadata("name", "Dex admin badge")
                .initial_supply(1);
            let state = 0u32;
            let component = Self {
                admin_badge: admin_badge.resource_address(),
                state: state,
            }
            .instantiate();

            let access_rules = AccessRules::new()
                .method("protected_update_state", rule!(require(admin_badge.resource_address())))
                .default(rule!(allow_all));

            (component.add_access_check(access_rules).globalize(), admin_badge)
        }

        pub fn update_state(&mut self, new_state: u32) -> u32 {
            let old_state = self.state;
            self.state = new_state;
            old_state
        }

        /// Protected update_state method, uses auth badge.
        pub fn protected_update_state(
            &mut self,
            new_state: u32
        ) -> u32 {
            let old_state = self.state;
            self.state = new_state;
            old_state
        }
    }
}
