#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nft {
    #[ink(storage)]
    pub struct NFT {
        owner: AccountId,
        id_to_owner: ink_storage::collections::HashMap<u64, AccountId>,
        owner_to_token_count: ink_storage::collections::HashMap<AccountId, u64>,
    }

    impl NFT {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                owner: Self::env().caller(),
                id_to_owner: ink_storage::collections::HashMap::new(),
                owner_to_token_count: ink_storage::collections::HashMap::new(),
            }
        }

        #[ink(message)]
        pub fn mint(&mut self, receiver: AccountId, token_id: u64) -> bool {
            self.id_to_owner.insert(token_id, receiver);
            let existing_number = self.owner_to_token_count.get(&receiver);
            if let Some(n) = existing_number {
                self.owner_to_token_count.insert(receiver, n + 1);
            } else {
                self.owner_to_token_count.insert(receiver, 1);
            }
            true
        }

        fn is_token_owner(&self, account: &AccountId, token_id: u64) -> bool {
            let owner = self.id_to_owner.get(&token_id);
            match owner {
                Some(acc) => return if acc != account {
                    false
                } else {
                    true
                },
                None => false,
            }
        }

        #[ink(message)]
        pub fn transfer(&mut self, from: AccountId, to: AccountId, token_id: u64) -> bool {
            if !self.is_token_owner(&from, token_id) {
                return false;
            }

            self.id_to_owner.insert(token_id, to);

            let from_owner_count = *self.owner_to_token_count.get(&from).unwrap_or(&0);
            let to_owner_count = *self.owner_to_token_count.get(&to).unwrap_or(&0);

            self.owner_to_token_count.insert(from, from_owner_count - 1);
            self.owner_to_token_count.insert(to, to_owner_count + 1);
            true
        }

        #[ink(message)]
        pub fn get_token_count_for_account(&self, account: AccountId) -> u64 {
            let number = self.owner_to_token_count.get(&account);
            if let None = number {
                return 0;
            }
            return *number.unwrap();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        use ink_lang as ink;

        #[ink::test]
        fn default_state() {
            //Given
            let nft = NFT::default();

            //When
            let tokens = nft.get_token_count_for_account(AccountId::from([0x1; 32]));

            //Then
            assert_eq!(tokens, 0);
        }

        #[ink::test]
        fn mint() {
            //Given
            let mut nft = NFT::default();
            let account_one = AccountId::from([0x1; 32]);
            let token_id = 95;

            //When
            nft.mint(account_one, token_id);

            //Then
            assert_eq!(nft.get_token_count_for_account(account_one), 1);
        }

        #[ink::test]
        fn transfer() {
            //Given
            let mut nft = NFT::default();
            let account_one = AccountId::from([0x1; 32]);
            let account_two = AccountId::from([0x2; 32]);
            let token_id = 95;

            //When
            nft.mint(account_one, token_id);
            nft.transfer(account_one, account_two, token_id);

            //Then
            assert_eq!(nft.get_token_count_for_account(account_one), 0);
            assert_eq!(nft.get_token_count_for_account(account_two), 1);
        }
    }
}
