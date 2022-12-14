#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod blindauction {

    /// the advantages of a blindauction is that 
    /// there is no time pressure towards the end 
    /// of the bidding period
    
    use ink_prelude::{
        vec,
        vec::Vec,
        string::String
    };
    use ink_storage::{traits::{SpreadAllocate, PackedLayout, SpreadLayout}, Mapping};
    
    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Bid {
        blinded_bid: String,
        deposit: u128,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Blindauction {
       beneficiary: AccountId,
       bidding_end: Timestamp,
       reveal_end: Timestamp,
       ended: bool,
       bids: Mapping<AccountId, Vec<Bid>>,
       highest_bidder: AccountId,
       highest_bid: u128,

       // allowed withdrwls of previous bids
       pending_returns: Mapping<AccountId, u128>,
    }

    #[ink(event)]
    pub struct AuctionEnded {
        #[ink(topic)]
        winner: AccountId,
        highest_bid: u128,
    }

    impl Blindauction {
        #[ink(constructor)]
        pub fn new(bidding_time: Timestamp, reveal_time: Timestamp, beneficary: AccountId) -> Self {
            let now = Self::env().block_timestamp();
            ink_lang::utils::initialize_contract(|contract: &mut Blindauction|{
                contract.bidding_end = now + bidding_time;
                contract.reveal_end = contract.bidding_end + reveal_time;
                contract.beneficiary = beneficary;
            })
        }

        #[ink(message, payable)]
        pub fn bid(&mut self, blinded_bid: String) {
            let now = self.env().block_timestamp();
            let deposit = self.env().transferred_value();
            let caller = self.env().caller();
            assert!(now < self.bidding_end);
            let bid = Bid {
                blinded_bid,
                deposit,
            };

            self.bids.insert(caller, &vec![bid]);
        }


        // #[ink(message)]
        // pub fn reveal(&mut self, values: Vec<i32>, fake: Vec<bool>, secret: Vec<String>) {
        //     let now = self.env().block_timestamp();
        //     let caller = self.env().caller();
        //     assert!(now < self.bidding_end);
        //     assert!(now > self.reveal_end);

        //     let _bids = self.bids.get(caller).unwrap_or_default();
        //     assert!(values.len() == _bids.len());
        //     assert!(fake.len() == _bids.len());
        //     assert!(secret.len() == _bids.len());

        //     let refund = 0;
        //     let i = 0;
        //     for i in i.._bids.len() as usize {
        //         let (value, fake, secret) = (values[i], fake[i], secret[i]);
                
        //     }
        // }
    }
}
