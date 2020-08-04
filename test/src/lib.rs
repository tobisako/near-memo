use borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Flipper {
    value: bool,
}

#[near_bindgen]
impl Flipper {
    pub fn get(&self) -> bool {
        return self.value;
    }

    pub fn flip(&mut self) {
        self.value = !self.value;
        let log_message = format!("fliped value to {}", self.value);
        env::log(log_message.as_bytes());
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "robert_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
        }
    }

    // mark individual unit tests with #[test] for them to be registered and fired
    #[test]
    fn increment() {
        // set up the mock context into the testing environment
        let context = get_context(vec![], false);
        testing_env!(context);
        // instantiate a contract variable with the counter at zero
        let mut contract = Flipper{ value: false };
        assert_eq!(false, contract.get());
        contract.flip();
        assert_eq!(true, contract.get());
        contract.flip();
        assert_eq!(false, contract.get());
    }
}
