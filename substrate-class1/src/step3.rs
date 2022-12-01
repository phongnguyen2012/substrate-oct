use std::collections::HashMap;


type AccountId = u64;
type VoteIndex = u64;

pub struct VotingModule {
    votes: HashMap<(AccountId, VoteIndex), bool>
}

impl VotingModule {
    pub fn new() -> Self {
        Self {
            votes: HashMap::new()
        }
    }
    pub fn vote(&mut self, account_id: AccountId, vote_index: VoteIndex, vote: bool) {
        self.votes.insert((account_id, vote_index), vote);
    
    }
    pub fn get_vote(&self, account_id: AccountId, vote_index: VoteIndex) -> bool {
        *self.votes.get(&(account_id, vote_index)).unwrap_or(&false)
    }
}