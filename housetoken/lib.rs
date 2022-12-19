#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod housetoken {
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};
    use openbrush::{
        contracts::psp34::extensions::metadata::*,
        traits::Storage
    };
    use ink_prelude::{
        // vec::Vec,
        vec,
        string::String
    };

    pub type HouseId = i32;

    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct House {
        id: HouseId,
        owner: AccountId,
        royalty_collector: AccountId,
        home_address: String,
        sq_feet: i32,
        bed_rooms: i32,
        bath_rooms: i32,
        price: u128,
        royalty: u128,
    }

    #[ink(storage)]
    #[derive(SpreadAllocate, Storage)]
    pub struct Housetoken {
        houses: Mapping<HouseId, House>,
        house_exists: Mapping<HouseId, bool>,
        next_id: i32,
        admin: AccountId,
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: Data,
    }

    #[ink(event)]
    pub struct BoughtHouse {
        #[ink(topic)]
        house_id: HouseId,
        seller: AccountId,
        buyer: AccountId
    }

    #[ink(event)]
    pub struct SentHouse {
        #[ink(topic)]
        house_id: HouseId,
        from: AccountId,
        to: AccountId
    }

    #[ink(event)]
    pub struct ChangedPrice {
        #[ink(topic)]
        house_id: HouseId,
        old_price: u128,
        new_price: u128,
    }

    // impl PSP34 for Housetoken {}

    impl Housetoken {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            ink_lang::utils::initialize_contract(|contract: &mut Self|{
                contract.admin = caller;
                contract.next_id = 1;
            })
        }


        #[ink(message)]
        pub fn create_house(&mut self, home_address: String, sq_feet: i32, bed_rooms: i32, bath_rooms: i32, price: u128, royalty: u128) {
            let caller = self.env().caller();
            let id = self.house_next_id();
            let exists_house = self.house_exists.get(id).unwrap_or_default();
            assert!(!exists_house, "this houde id already exists");
            assert!(self.admin == caller, "must be admin to mint house token");
            assert!(royalty <= 1000, "cannot have royalty more than 100%");

            let house = House {
                id,
                owner: caller,
                royalty_collector: caller,
                home_address,
                sq_feet,
                bath_rooms,
                bed_rooms,
                price,
                royalty,
            };

            self.houses.insert(id, &house);
            self._mint_to(caller, Id::U8(id as u8)).unwrap_or_default();
            self.house_exists.insert(id, &true);
        }

        #[ink(message)]
        pub fn get_house(&self, id: i32) -> Option<House> {
            self.houses.get(id)
        }

        #[ink(message)]
        pub fn change_price(&mut self, id: i32, new_price: u128) {
            let exist_house = self.house_exists.get(id).unwrap_or_default();
            let mut house = self.houses.get(id).unwrap_or_default();
            assert!(exist_house, "houde must exists");

            let old_price = house.price;
            house.price = new_price;
            self.env().emit_event(ChangedPrice {house_id: id, old_price, new_price});
        }

        #[ink(message, payable)]
        pub fn buy_house(&mut self, id: i32) {
            let caller = self.env().caller();
            // let value = self.env().transferred_value();
            let exist_house = self.house_exists.get(id).unwrap_or_default();
            let mut house = self.houses.get(id).unwrap_or_default();
            assert!(exist_house, "houde must exists");

            assert!(self._owner_of(&Id::U8(id as u8)) != Some(caller), "cannot buy your own house");
            // assert!(value == house.price, "not enough balance");
            let _seller = house.owner;
            let _royalty_collector = house.royalty_collector;
            let royalty_payment = house.royalty * (house.price / 1000);
            let seller_payment = house.price - royalty_payment;

            // pay the seller
            self.env().transfer(_seller, seller_payment).unwrap_or_default();
            self.env().transfer(_royalty_collector, royalty_payment).unwrap_or_default();

            // send house token to new owner
            self._transfer_token(caller, Id::U8(id as u8), vec![]).unwrap_or_default();
            //change the owneership of house
            house.owner = caller;
            self.env().emit_event(BoughtHouse{house_id: id, seller: _seller, buyer: caller});



        }

        fn house_next_id(&mut self) -> HouseId {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }
    
}
