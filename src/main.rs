// Step 1: Create a Trait called Account
trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

// Step 2: Implement the Account Trait for a struct called BankAccount
struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

// Step 3: Implement deposit method for BankAccount
impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Step 4: Implement withdraw method for BankAccount
    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient balance");
        }
    }

    // Step 5: Implement balance method for BankAccount
    fn balance(&self) -> f64 {
        self.balance
    }
}

fn main() {
    // Step 6: Create two BankAccount instances with different details
    let mut  account1 = BankAccount {
        account_number: 1001,
        holder_name: String::from("John"),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 1002,
        holder_name: String::from("Alice"),
        balance: 1500.0,
    };

    // Step 7: Deposit and withdraw from accounts
    account1.deposit(500.0);
    account2.withdraw(300.0);

    // Step 8: Call the balance method on both accounts and print the result
    println!("Account 1 Balance: {}", account1.balance());
    println!("Account 2 Balance: {}", account2.balance());
}
