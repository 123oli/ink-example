#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod openauction {
    use ink_storage::{Mapping, traits::SpreadAllocate};


    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Openauction {
        beneficiary: AccountId,
        auction_end_time: Timestamp,

        // current state of the auction
        highest_bidder: AccountId,
        highest_bid: u128,

        // allowed withdrawals of previous bids
        pending_returns: Mapping<AccountId, u128>,

        // set to true at the end, disallows any change.
        // by default initialized to `false`
        ended: bool,
    }

    // events that will be emitted on changes
    #[ink(event)]
    pub struct HighestBidIncreased {
        #[ink(topic)]
        bidder: AccountId,
        amount: u128
    }

    #[ink(event)]
    pub struct AuctionEnded {
        #[ink(topic)]
        winner: AccountId,
        amount: u128
    }

    impl Openauction {
        /// create a simple auction with `bidding_time`
        /// seconds bidding time on behalf of the
        /// beneficiary address `beneficiary`
        #[ink(constructor)]
        pub fn new(bidding_time: Timestamp, beneficiary: AccountId) -> Self {
            let now = Self::env().block_timestamp();
            ink_lang::utils::initialize_contract(|_contract: &mut Openauction|{
                _contract.beneficiary = beneficiary;
                _contract.auction_end_time = now + bidding_time;
            })
        }

        /// bid on the auction with the value sent
        /// together with the transactions.
        /// The value will only be refunded if the 
        /// auction is not won.
        #[ink(message, payable)]
        pub fn bid(&mut self) {
            // revert the call if the bidding period is over
            let now = self.env().block_timestamp();
            let caller = self.env().caller();
            assert!(now <= self.auction_end_time, "auction already ended.");

            // if the bid is not higher, send the money back
            let balance = self.env().transferred_value();
            assert!(balance > self.highest_bid, "there already is higher bid.");

            if self.highest_bid != 0 {
                let mut _pending_returns = self.pending_returns.get(self.highest_bidder).unwrap_or_default();
                _pending_returns += self.highest_bid;
            }

            self.highest_bidder = caller;
            self.highest_bid = balance;
            self.env().emit_event(HighestBidIncreased{
                bidder: caller,
                amount: balance
            });
        }

        /// withdraw a bid that was overbid
        #[ink(message)]
        pub fn withdraw(&mut self) -> bool {
            let caller = self.env().caller();
            let amount = self.pending_returns.get(caller).unwrap_or_default();
            // it is important to set this to zero because the recipent
            // can call this function again as part of the receiving call
            // before `send` returns.
            if amount > 0 {
                self.pending_returns.insert(caller, &0);

                // let transfer = self.env().transfer(caller, amount);
                let result = match self.env().transfer(caller, amount) {
                    Ok(()) => true,
                    Err(_) => false
                };
                if !result {
                    // no need to call throw here, just reset the amount owing
                    self.pending_returns.insert(caller, &amount);
                    false;
                }
            }
            true
        }

        /// end the auction and send the highest bid
        /// to the beneficiary
        #[ink(message)]
        pub fn auction_end(&mut self) {
            let now = self.env().block_timestamp();
            assert!(now >= self.auction_end_time, "auction not yet ended.");
            assert!(!self.ended, "auction end has already been called.");

            self.ended = true;
            self.env().emit_event(AuctionEnded{
                winner: self.highest_bidder,
                amount: self.highest_bid,
            });

            self.env().transfer(self.beneficiary, self.highest_bid).unwrap_or_default();
        }
    }

}
