use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Counter {
    counter: i128,
}

impl Default for Counter {
    fn default() -> Self {
        Self { counter: 5 }
    }
}

#[near_bindgen]
impl Counter {
    pub fn get_counter(&self) -> i128 {
        self.counter.clone()
    }

    pub fn set_counter(&mut self, value: i128) {
        log!("This is a new counter {}", value);
        self.counter = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // We expect of this function to return to us the value that we are requesting. If the default value is 5 and we are calling gegit initt_counter, we expect of the contract to return 5 to us .
    fn get_counter() {
        // So first we have to instantiate our contract with the name of our contract by calling its default function
        // The default function for our contract will set the state variable "counter" to 5 .
        let contract = Counter::default();

        // After setting our state variable "counter we can check equality by assert_eq
        assert_eq!(contract.get_counter(), 5)
    }

    #[test]
    // We expect of this function to return to us the value that we just updated. If we call the set_counter function then the counter variable will be updated .
    // Thats why if we call get_counter the value that needs to be returned is the value that we have just changed .
    fn set_counter() {
        // We called mut here because we will update a new value to the struct of the contract . So that's why we need to mutate it (change it)
        let mut contract = Counter::default();
        // We set the counter to 55
        contract.set_counter(55);
        // Now the counter needs to be verified if it has changed or not. We will check equality for that.
        assert_eq!(contract.get_counter(), 55);
    }
}
