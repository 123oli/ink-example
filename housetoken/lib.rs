#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod housetoken {
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};
    use openbrush::contracts::traits::psp34::extensions::mintable::*;
    use ink_prelude::string::String;

    pub type HouseId = i32;

    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct House {
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
    #[derive(SpreadAllocate)]
    pub struct Housetoken {
        houses: Mapping<HouseId, House>,
        house_exists: Mapping<HouseId, bool>,
        next_id: i32,
        admin: AccountId,
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

    impl Housetoken {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            ink_lang::utils::initialize_contract(|contract: &mut Self|{
                contract.admin = caller;
            })
        }


        #[ink(message)]
        pub fn mint(&self, home_address: String, sq_feet: i32, bed_rooms: i32, bath_rooms: i32, price: u128, royalty: u128) {
            let caller = self.env().caller();
            let house_id = self.house_next_id();
            let exists_house = self.house_exists.get(house_id).unwrap_or_default();
            assert!(!exists_house, "this houde id already exists");
            assert!(self.admin = caller, "must be admin to mint house token");
            assert!(royalty <= 1000, "cannot have royalty more than 100%");

            let house = House {
                owner: caller,
                royalty_collector: caller,
                home_address,
                sq_feet,
                bath_rooms,
                price,
                royalty,
            };

            self.houses.insert(houde_id, &house);
            PSP34MintableRef::mint(caller, Id::U8(houde_id))?;
            self.house_exists.insert(houde_id, &true);
        }

        #[ink(message)]
        pub fn change_price(&mut self, houde_id: i32, new_price: u128) {
            let exist_house = self.house_exists.get(houde_id).unwrap_or_default();
            let mut house = self.houses.get(house_id).unwrap_or_default();
            assert!(exist_house, "houde must exists");

            let old_price = house.price;
            house.price = new_price;
            self.env().emit_event(ChangedPrice {house_id, old_price, new_price});
        }

        fn house_next_id(&mut self) -> HouseId {
            let id = self.next_id;
            self.next_id += 1;
            id
        }
    }
    
}
