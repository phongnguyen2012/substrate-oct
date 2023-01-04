use std::collections::HashMap;


type AccountId = u32;
type Balance = u32;
pub struct BalancesModule {
    balances: HashMap<AccountId, Balance>,
}
impl BalancesModule {
    pub fn new() -> Self {
        Self{
            balances: HashMap::new(),
        }
    }
    pub fn set_balance(&mut self, account_id: AccountId, balance: Balance) {
        self.balances.insert(account_id, balance);
    }
    pub fn transfer(&mut self, from: AccountId, to: AccountId, amount: Balance) -> Result<(), &'static str> {
        let from_balance = self.balances.get(&from).ok_or("account not found")?;
        let to_balance = self.balances.get(&to).unwrap_or(&0);
        let new_from_balance = from_balance.checked_sub(amount).ok_or("balance too low")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("balance too high")?;
        //ok_or()? ? de unwrap
        self.balances.insert(from, new_from_balance);
        self.balances.insert(to, new_to_balance);
        Ok(())
    }
    pub fn get_balance(&self, account_id: AccountId) -> &Balance {
        self.balances.get(&account_id).unwrap()
    }
}