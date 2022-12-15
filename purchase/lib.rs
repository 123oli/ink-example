#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod purchase {
    use ink_storage::traits::SpreadLayout;
    use openbrush::{
        modifier_definition,
        modifiers
    };


    #[derive(SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, scale::Decode, scale::Encode, Debug, ink_storage::traits::StorageLayout))]
    pub enum State {
        CREATED,
        LOCKED,
        INACTIVE,
    }

    #[ink(storage)]
    pub struct Purchase {
        value: u128,
        seller: AccountId,
        buyer: AccountId,
        state: State,
    }

    #[ink(event)]
    pub struct Aborted {}

    #[ink(event)]
    pub struct PurchaseConfirmed {}

    #[ink(event)]
    pub struct ItemReceived {}

    impl Purchase {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let value = Self::env().transferred_value();

            assert!((2 * value) == value , "value has to be even.");
            Self { 
                value: value / 2,
                seller: caller,
                buyer: Default::default(),
                state: State::CREATED,
             }
        }

        // abort the purchase and reclaim the value.
        // can only be called by the seller before
        // the contract is locked
        #[ink(message, payable)]
        pub fn abort(&mut self) {
            let caller = self.env().caller();
            let balance = self.env().transferred_value();
            assert!(caller == self.seller, "only seller can call this");
            assert!(self.state == State::CREATED, "invalid state");
            self.env().emit_event(Aborted{});
            self.state = State::INACTIVE;

            self.env().transfer(self.seller, balance);
         }

         // confirm the purchase as buyer.
         // transaction has to include `2 * value` amount
         // the value will be locked until confirmereceived is called
         #[ink(message, payable)]
         pub fn confirm_purchase(&mut self) {
            let value = self.env().transferred_value();
            assert!(self.state == State::CREATED, "invalid state");
            assert!(value == (2*self.value));

            self.env().emit_event(PurchaseConfirmed{});
            self.buyer = caller;
            self.state = State::LOCKED;
         }

         // confirm that you received the item
         // this will release the locked ether
         #[ink(message, payable)]
         pub fn confirm_received(&mut self) {
            let caller = self.env().caller();
            let balance = self.env().transferred_value();
            assert!(caller == self.buyer, "only seller can call this");
            assert!(self.state == State::LOCKED, "invalid state");

            self.env().emit_event(ItemReceived{});

            // it is important to change the state first because
            // otherwise the contracts called using send below
            // can call in again here
            self.state = State::INACTIVE;

            // this actually allows both the buyer and the seller
            // to block the reufnd - the withdraw pattern should be used

            self.env().transfer(self.buyer, self.value);
            self.env().transfer(self.seller, balance);

         }
    }

    // #[modifier_definition]
    // pub fn only_fee_setter<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
    // where
    //     T: Storage<data::Data>,
    //     F: FnOnce(&mut T) -> Result<R, E>,
    //     E: From<FactoryError>,
    // {
    //     if instance.data().fee_to_setter != T::env().caller() {
    //         return Err(From::from(FactoryError::CallerIsNotFeeSetter))
    //     }
    //     body(instance)
    // }
}
