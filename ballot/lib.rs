#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod ballot {

    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::{Mapping, traits::{SpreadAllocate, PackedLayout, SpreadLayout}};

    /// This declares a new complex type which will
    /// be used for vairables later.
    /// It will represent a single voter.
    #[derive(scale::Decode, scale::Encode, PackedLayout, SpreadLayout, Default)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Voter {
        weight: u64,
        voted: bool,
        delegate: AccountId,
        vote: i32
    }

    // This is a type of single proposal.
    #[derive(scale::Decode, scale::Encode, Default, Debug, PackedLayout, SpreadLayout, SpreadAllocate, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Proposal {
        name: String,
        vote_count: u64,
    }

    impl ink_storage::traits::PackedAllocate for Proposal {
        fn allocate_packed(&mut self, at: &ink_primitives::Key){
            ink_storage::traits::PackedAllocate::allocate_packed(&mut *self, at)
        }
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[derive(SpreadAllocate)]
    #[ink(storage)]
    pub struct Ballot {
        chair_person: AccountId,
        // This declares a state variable that
        // Stores a `Voter` struct for each possiable address
        voters: Mapping<AccountId, Voter>,
        // A dynamically-sized array of `Proposal` structs.
        proposals: Vec<Proposal>,
    }

    impl Ballot {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(proposal_name: Vec<String>) -> Self {
            let caller = Self::env().caller();
            ink_lang::utils::initialize_contract(|contract: &mut Ballot|{
                contract.chair_person = caller;
                let mut voter = contract.voters.get(caller).unwrap_or_default();
                voter.weight = 1;

                // For each of the provided proposal names,
                // create a new proposal object and add it
                // to the end of the array
                let i = 0;
                for i in i..=proposal_name.len(){
                    let proposal = Proposal {
                        name: proposal_name[i].clone(),
                        vote_count: 0,
                    };
                    contract.proposals.push(proposal);
                }
            })
        }

        // Give `voter` the right to vote on this ballot.
        // May only be called by `chairperson`
        #[ink(message)]
        pub fn give_right_to_vote(&mut self, voter: AccountId) {
            let caller = self.env().caller();
            assert!(caller == self.chair_person, "Only chairperson can give right to vote");

            let mut voters = self.voters.get(voter).unwrap_or_default();
            assert!(!voters.voted, "The voter already voted");
            assert!(voters.weight == 0);
            voters.weight = 1;
        }

        // Delegeate your vote to the voter `to`

        // #[ink(message)] 
        #[inline]
        pub fn delegate(&mut self, to: &mut AccountId) {
            let caller = self.env().caller();
            let mut voters = self.voters.get(caller).unwrap_or_default();
            let mut voters_to = self.voters.get(*to).unwrap_or_default();
            assert!(!voters.voted, "You already voted.");

            assert!(*to != caller, "Self-delegation is disallowed");

            // Forward the delegation as long as 
            // `to` also delegated
            // In general, such loops are very dangerous,
            // because if they run too long, they might need more gas than is available in block.
            // In this case, the delegation will not be execuated,
            // but in other situations, such loops might
            // cause a contract to get "stuck" completely

            while voters_to.delegate != [0; 32].into() {
                *to = voters_to.delegate;

                // We found a loop in the delegation, not allowed.
                assert!(*to != caller, "Found loop in delegation");
            }

            // Since `sender` is a reference, this
            // modifies `voters.voted`
            voters.voted = true;
            voters.delegate = *to;

            if voters_to.voted {
                // If the delegate already voted,
                // directly add to the number of votes
                let mut _vote_count = self.proposals[voters_to.vote as usize].vote_count;
                _vote_count += voters.weight;
            } else {
                voters_to.weight += voters.weight;
            }

        }

        // Give your vote (including votes delegated to you)
        // to proposal `proposals[proposal].name`
        #[ink(message)]
        pub fn vote(&mut self, proposal: i32) {
            let caller = self.env().caller();
            let mut voters = self.voters.get(caller).unwrap_or_default();
            assert!(voters.weight != 0, "Has no right to vote");
            assert!(!voters.voted, "Already voted.");
            voters.voted = true;
            voters.vote = proposal;

            // if `proposal` is out of the range of the array,
            // this will throw automatically and revert all changes
            let mut _vote_count = self.proposals[proposal as usize].vote_count;
            _vote_count += voters.weight;
        }

        // Computes the winning proposal taking all
        // previous votes into account
        #[ink(message)]
        pub fn winning_proposal(&self) -> i32 {
            let mut winning_vote_count = 0;
            let mut _winning_proposal = 0;
            let i = 0;
            for i in i..self.proposals.len() as usize {
                if self.proposals[i].vote_count > winning_vote_count {
                    winning_vote_count = self.proposals[i].vote_count;
                    _winning_proposal = i as i32;
                }
            }

            _winning_proposal
        }

        // Calls `winningProposal()` function to get the index
        // of the winner contained in the proposals array and then 
        // returns the name of the winner
        #[ink(message)]
        pub fn winner_name(&self) -> String {
            self.proposals[self.winning_proposal() as usize].name.clone()
        }

    }
}
