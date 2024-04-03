use rust_typestate::PasswordManagerBuilder;

/// Demonstration of the API in use.  Once again this is an EXAMPLE and not designed for real-world use.
fn main() {
    // A builder pattern to easily create a password manager and add new passwords to it.
    // Try calling `.build()` without setting a master password.
    let mut manager = PasswordManagerBuilder::new()
        .with_master_password("Hunter2")
        .with_account("test@example.com", "Bees123")
        .with_account("person@social.com", "Wasps456")
        .with_account("me@news.biz", "Hornets789")
        .build();

    // Below is a simple command line interface to show how this might be used.

    // A simple loop to allow the user 3 attempts to enter their password correctly before exiting the program.
    //
    // This could be refactored into a function that returns a concrete type of [PasswordManager<Unlocked>] as the program is quit if the user does not
    // enter the correct password within the given guesses.
    let mut remaining_attempts = 3;
    let unlocked_manager = loop {
        if remaining_attempts == 0 {
            println!("Too many incorrect password attempts!");
            std::process::exit(0);
        }
        remaining_attempts -= 1;

        // Get the user's password attempt.
        let mut password_input = String::new();
        println!("Enter the master password: ");
        std::io::stdin()
            .read_line(&mut password_input)
            .expect("Failed to read line from stdin.");

        match manager.unlock(password_input.trim()) {
            // If the manager unlocks we break on it, returning the value from the loop.
            Ok(unlocked) => break unlocked,
            // If the manager is still locked we have to replace the original variable with it as ".unlock()" consumes self.
            // This is a weird quirk of the API but you can't conditionally choose between moving and taking a reference of self.
            Err(still_locked) => manager = still_locked,
        }
    };

    // Get the user to input the name of the account they'd like to retrieve the password for.
    let mut account_input = String::new();
    println!("Enter the name of the account: ");
    std::io::stdin()
        .read_line(&mut account_input)
        .expect("Failed to read line from stdin.");
    let account_input = account_input.trim();

    // Get the password for that account (if one exists).
    println!("Getting password for account {account_input:?}...");
    let social_password = unlocked_manager.get_password(account_input);
    match social_password {
        Some(ref password) => println!("Password is {password:?}."),
        None => println!("Looks like there isn't a password associated with that account."),
    }

    // Lock the password manager for safe keeping.
    println!("Locking password manager...");
    let _ = unlocked_manager.lock();
    println!("Password manager locked. Have a nice day.");
}
