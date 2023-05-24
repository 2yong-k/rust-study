use ink_lang as ink;

#[ink::contract]
mod token {
    #[ink(storage)]
    pub struct Token {
        total_supply: Balance,
        balances: ink_storage::collections::HashMap<AccountId, Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }

    impl Token {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = ink_storage::collections::HashMap::new();
            let caller = Self::env().caller();
            balances.insert(caller, total_supply);
            Self {
                total_supply,
                balances,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> bool {
            let caller = self.env().caller();
            let balance = self.balances.get(&caller).copied().unwrap_or(0);

            if balance < value {
                return false;
            }

            self.balances.insert(caller, balance - value);
            let to_balance = self.balances.get(&to).copied().unwrap_or(0);
            self.balances.insert(to, to_balance + value);

            self.env().emit_event(Transfer {
                from: Some(caller),
                to: Some(to),
                value,
            });

            true
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).copied().unwrap_or(0)
        }
    }
}
