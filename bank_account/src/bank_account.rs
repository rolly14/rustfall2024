#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount
        {
            balance: initial_balance,
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        self.balance -= amount;
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let initial_balance = 2300.0;
        let account = BankAccount::new(initial_balance);
        assert_eq!(account.balance(), initial_balance);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let initial_balance = 2300.0;
        let mut account = BankAccount::new(initial_balance);
        account.deposit(250.0);
        assert_eq!(account.balance(), 2550.0);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let initial_balance = 2300.0;
        let mut account = BankAccount::new(initial_balance);
        account.withdraw(100.0);
        assert_eq!(account.balance(),2200.0);
    }

    // Add more tests here
}