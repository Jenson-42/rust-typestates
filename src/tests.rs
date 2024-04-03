//! Testing the password manager.

use crate::password_manager::PasswordManagerBuilder;

/// Test that unlocking the password manager using the same master password it was created with actually unlocks it.
#[test]
fn unlocking_manager_with_correct_password_works() {
    const MASTER_PASSWORD: &str = "Master Password";

    let manager = PasswordManagerBuilder::new()
        .with_master_password(MASTER_PASSWORD)
        .build();

    assert!(manager.unlock(MASTER_PASSWORD).is_ok());
}

/// Test that unlocking the password manager using the same master password it was created with actually unlocks it.
#[test]
fn unlocking_manager_with_incorrect_password_fails() {
    const MASTER_PASSWORD: &str = "Master Password";

    let manager = PasswordManagerBuilder::new()
        .with_master_password(MASTER_PASSWORD)
        .build();

    let incorrect_master_password = format!("Not {MASTER_PASSWORD}");

    assert!(manager.unlock(incorrect_master_password).is_err());
}

/// Ensure retrieval of passwords known to be stored works.
#[test]
fn retrieving_present_password_unlocked_manager_works() {
    const MASTER_PASSWORD: &str = "Master Password";
    const ACCOUNT: &str = "Account";
    const PASSWORD: &str = "Hunter2";

    let manager = PasswordManagerBuilder::new()
        .with_master_password(MASTER_PASSWORD)
        .with_account(ACCOUNT, PASSWORD)
        .build()
        .unlock(MASTER_PASSWORD)
        .expect("Unlocking with correct master password should work");

    let retrieved_password = manager.get_password(ACCOUNT);

    assert_eq!(retrieved_password, Some(String::from(PASSWORD)));
}

/// Ensure retrieval of passwords known not to be stored fails.
#[test]
fn retrieving_nonexistant_password_fails() {
    const MASTER_PASSWORD: &str = "Master Password";

    let manager = PasswordManagerBuilder::new()
        .with_master_password(MASTER_PASSWORD)
        .build()
        .unlock(MASTER_PASSWORD)
        .expect("Unlocking with correct password should work");

    let retrieved_password = manager.get_password("Not an Account");

    assert_eq!(retrieved_password, None);
}
