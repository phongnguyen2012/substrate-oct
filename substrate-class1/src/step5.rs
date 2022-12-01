
use std::hash::Hash;
use std::collections::HashMap;

use num::traits::{Zero, CheckedAdd, CheckedSub};
pub trait Config {
    type AccountId: Eq + Hash;
    type VoteIndex: Eq + Hash;
    type Balance: Zero+CheckedAdd+CheckedSub+Copy;
}
pub struct BalancesModule<T: Config> {
    balances: HashMap<T::AccountId, T::Balance>,
}

// impl Hash for AccountId (Generic type) -> impl Hash for u32, impl Hash for u64, impl Hash for u128
// impl Eq for AccountId (Generic type) -> impl Eq for u32, impl Eq for u64, impl Eq for u128
impl<T: Config> BalancesModule<T> {
    pub fn new() -> Self {
        Self{
            balances: HashMap::new(),
        }
    }
    pub fn set_balance(&mut self, account_id: T::AccountId, balance: T::Balance) {
        self.balances.insert(account_id, balance);
    }
    pub fn transfer(&mut self, from: T::AccountId, to: T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        let from_balance = self.balances.get(&from).ok_or("account not found")?;
        let zero_balance = T::Balance::zero();
        let to_balance = self.balances.get(&to).unwrap_or(&zero_balance);
        let new_from_balance = from_balance.checked_sub(&amount).ok_or("balance too low")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("balance too high")?;
        //ok_or()? ? de unwrap
        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);
        Ok(())
    }
    pub fn get_balance(&self, account_id: T::AccountId) -> T::Balance {
        *self.balances.get(&account_id).unwrap_or(&T::Balance::zero())
    }
}