#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod openauction {
    use ink_storage::Mapping;


    #[ink(storage)]
    pub struct Openauction {
        beneficiary: AccountId,
        auction_end_time: Timestamp,

        // Current state of the auction
        highest_bidder: AccountId,
        highest_bid: u64,

        // Allowed withdrawals of previous bids
        pending_returns: Mapping<AccountId, u64>,

        // Set to true at the end, disallows any change.
        // By default initialized to `false`
        ended: bool,
    }

    // Events that will be emitted on changes

    impl Openauction {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn get(&mut self) {}
    }

}
