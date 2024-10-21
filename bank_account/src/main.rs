mod bank_account;  

fn main() {
    let mut account = bank_account::BankAccount::new(2300.0);
   
    account.deposit(250.0);
    account.withdraw(100.0);
}
