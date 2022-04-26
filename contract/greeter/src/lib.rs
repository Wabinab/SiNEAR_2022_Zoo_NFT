use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, require};
use near_sdk::collections::LookupMap;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Welcome {
    records: LookupMap<AccountId, String>,
    others: LookupMap<AccountId, String>,
}

impl Default for Welcome {
    fn default() -> Self {
      Self {
        records: LookupMap::new(b"a".to_vec()),
        others: LookupMap::new(b"b".to_vec()),
      }
    }
}


#[near_bindgen]
impl Welcome {
    /// Change method: set the greeting for signer account. 
    pub fn set_greeting(&mut self, message: String) {
      let account_id = env::signer_account_id();

      env::log_str(
        format!(
          "Hello, {} from {}",
          message,
          account_id
        ).as_str(),
      );

      self.records.insert(&account_id, &message);
    }


    /// View method: get the greeting for a particular account. 
    pub fn get_greeting(&self, account_id: AccountId) -> String {
      match self.records.get(&account_id) {
        Some(greeting) => greeting,
        None => "".to_owned(),
      }
    }

    
    /// Anyone could set a greeting for some other target accounts. 
    /// These are independent from the greeting set for themselves. 
    /// For simplicity, we won't ask for permission from owner to 
    /// accept/reject a greeting set. 
    pub fn set_greeting_for_others(
      &mut self, 
      target: AccountId,
      message: String,
    ) {
      let signer = env::signer_account_id();

      require!(
        signer != target,
        "Only others can set a greeting for you."
      );

      env::log_str(
        format!(
          "@{} say {} to @{}",
          signer,
          message,
          target
        ).as_str(),
      );

      self.others.insert(&target, &message);
    }

    pub fn get_others_set_greeting(&self, account_id: AccountId) -> String {
      match self.others.get(&account_id) {
        Some(greeting) => greeting,
        None => "".to_owned(),
      }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{VMContext, testing_env};
    use near_sdk::test_utils::{VMContextBuilder};


    fn get_context(is_view: bool) -> VMContext {
      VMContextBuilder::new()
          .current_account_id("alice_near".parse().unwrap())
          .signer_account_id("bob_near".parse().unwrap())
          .predecessor_account_id("carol_near".parse().unwrap())
          .is_view(is_view)
          .build()
    }

    #[test]
    fn test_set_and_get_greeting() {
      let context = get_context(false);
      testing_env!(context);

      let mut contract = Welcome::default();
      contract.set_greeting("howdy".to_owned());

      assert_eq!(
        "howdy".to_owned(),
        contract.get_greeting("bob_near".parse().unwrap())
      );
    }

    #[test]
    fn test_get_default_greeting() {
      let context = get_context(true);
      testing_env!(context);

      let contract = Welcome::default();

      assert_eq!(
        "".to_owned(),
        contract.get_greeting("francis_near".parse().unwrap())
      );
    }

    #[test]
    fn test_set_greeting_from_others_and_get() {
      let context = get_context(false);
      let signer_id: String = context.signer_account_id.clone();
      testing_env!(context);

      let target: AccountId = "francis_near".parse().unwrap();

      let mut contract = Welcome::default();
      contract.set_greeting_for_others(target.clone(), "NEAR".to_owned());

      assert_eq!(
        "NEAR".to_owned(),
        contract.get_others_set_greeting(target)
      );

      assert_ne!(
        "NEAR".to_owned(),
        contract.get_others_set_greeting(signer_id.parse().unwrap())
      );
    }

    #[test]
    #[should_panic(expected = "Only others can set a greeting for you.")]
    fn test_cannot_set_greeting_for_ourselves() {
      let context = get_context(false);
      let target: AccountId = context.signer_account_id.clone().parse().unwrap();
      testing_env!(context);

      let mut contract = Welcome::default();
      contract.set_greeting_for_others(target, "Howdy".to_owned());
    }
}
