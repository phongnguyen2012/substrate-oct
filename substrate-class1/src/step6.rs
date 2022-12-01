use std::hash::Hash;
use std::collections::HashMap;

pub trait Config  {
    type AccountId: Eq + Hash;
    type VoteIndex: Eq + Hash;
    
}

pub struct VotingModule<T: Config> {
    votes: HashMap<(T::AccountId, T::VoteIndex), bool>
}

impl <T: Config> VotingModule<T> {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new()
        }
    }
    pub fn vote(&mut self, account_id:T::AccountId, vote_index: T::VoteIndex, vote: bool) {
        self.votes.insert((account_id, vote_index), vote);
    
    }
    pub fn get_vote(&self, account_id: T::AccountId, vote_index: T::VoteIndex) -> bool {
        *self.votes.get(&(account_id, vote_index)).unwrap_or(&false)
    }
}
