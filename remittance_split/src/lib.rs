#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct RemittanceSplit;

#[contractimpl]
impl RemittanceSplit {
    /// Initialize a remittance split configuration
    /// 
    /// # Arguments
    /// * `spending_percent` - Percentage for daily spending (0-100)
    /// * `savings_percent` - Percentage for savings (0-100)
    /// * `bills_percent` - Percentage for bills (0-100)
    /// * `insurance_percent` - Percentage for insurance (0-100)
    /// 
    /// # Returns
    /// Returns true if percentages sum to 100, false otherwise
    pub fn initialize_split(
        env: Env,
        spending_percent: u32,
        savings_percent: u32,
        bills_percent: u32,
        insurance_percent: u32,
    ) -> bool {
        let total = spending_percent + savings_percent + bills_percent + insurance_percent;
        
        if total != 100 {
            return false;
        }
        
        // Store the split configuration
        env.storage().instance().set(
            &symbol_short!("SPLIT"),
            &vec![
                &env,
                spending_percent,
                savings_percent,
                bills_percent,
                insurance_percent,
            ],
        );
        
        true
    }
    
    /// Get the current split configuration
    /// 
    /// # Returns
    /// Vec of percentages: [spending, savings, bills, insurance]
    pub fn get_split(env: Env) -> Vec<u32> {
        env.storage()
            .instance()
            .get(&symbol_short!("SPLIT"))
            .unwrap_or_else(|| vec![&env, 50, 30, 15, 5]) // Default split
    }
    
    /// Calculate split amounts from a total remittance amount
    /// 
    /// # Arguments
    /// * `total_amount` - Total remittance amount in smallest unit
    /// 
    /// # Returns
    /// Vec of amounts: [spending, savings, bills, insurance]
    pub fn calculate_split(env: Env, total_amount: i128) -> Vec<i128> {
        let split = Self::get_split(env);
        
        let spending = (total_amount * split.get(0).unwrap() as i128) / 100;
        let savings = (total_amount * split.get(1).unwrap() as i128) / 100;
        let bills = (total_amount * split.get(2).unwrap() as i128) / 100;
        let insurance = total_amount - spending - savings - bills; // Remainder to handle rounding
        
        vec![&env, spending, savings, bills, insurance]
    }
}

#[cfg(test)]
mod test;

