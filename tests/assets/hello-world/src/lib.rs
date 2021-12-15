use scrypto::prelude::*;

blueprint! {
    struct Hello {
        state: u32,
    }

    impl Hello {
        pub fn new(state: u32) -> Component {
            Self { state }.instantiate()
        }

        pub fn update_state(&mut self, new_state: u32) -> u32 {
            let old_state = self.state;
            self.state = new_state;
            old_state
        }
    }
}
