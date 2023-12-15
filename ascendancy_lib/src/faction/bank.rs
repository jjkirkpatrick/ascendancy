use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

/// The factions bank balance
#[derive(Component, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bank {
    /// The amount of money the faction has
    pub balance: u32,

    /// The total deposits made by the faction
    pub total_deposits: u32,

    /// The total withdrawals made by the faction
    pub total_withdrawals: u32,

    /// The total loans taken by the faction
    pub total_loans: u32,

    /// The total loans repaid by the faction
    pub total_loans_repaid: u32,
}

impl Bank {
    /// create a new bank with a random balance between 1000000 and 2000000
    pub fn randomized() -> Self {
        Self {
            balance: rand::thread_rng().gen_range(1000000..2000000),
            total_deposits: 0,
            total_withdrawals: 0,
            total_loans: 0,
            total_loans_repaid: 0,
        }
    }

    /// Get the balance of the bank
    pub fn bank_balance(&self) -> u32 {
        self.balance
    }

    /// Deposit money into the bank
    pub fn deposit(&mut self, amount: u32) {
        self.balance += amount;
        self.total_deposits += amount;
    }

    /// Withdraw money from the bank
    pub fn withdraw(&mut self, amount: u32) {
        self.balance -= amount;
        self.total_withdrawals += amount;
    }
}
