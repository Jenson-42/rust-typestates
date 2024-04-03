//! This is an *EXAMPLE* password manager to demonstrate how Rust's type system can be leveraged to make APIs more robust and even transmute some runtime
//! errors into compile-time errors.  It is *NOT* intended to be used for password management as it has not been designed with security in mind.

// The PhantomData type allows us to add generic types to structs without actually using them in the struct.  It is a Zero-Sized type meaning it is
// optimised away by the Rust compiler and only exists to benefit the developer.
use core::marker::PhantomData;
use std::collections::HashMap;

/// Denotes a locked [PasswordManager].
#[derive(Debug)]
pub struct Locked;
/// Denotes an unlocked [PasswordManager].
#[derive(Debug)]
pub struct Unlocked;

/// The password manager struct.
///
/// Instead of embedding the locked state using a boolean field on the struct, it is implemented as a generic type.
/// This has the benefit of there being no extra memory usage incurred by storing the state as a field within the struct itself, but the amount of memory
/// taken by a single boolean value is negligable in most cases.
///
/// The main reason for this choice is to turn some runtime errors into compile time errors.  Instead of each method implemented for this struct having to
/// check the manager's locked state and reacting to it (for example a .get_password(..) method having to return a [Result] in case the manager is locked),
/// methods are only implemented on the struct for the states they apply to.  This simplifies the return type of the methods, reduces code duplication, and
/// only exposes an API of methods that are valid to call given the current state.
///
/// The fields all being private also prevents accidentally leaking passwords from locked managers by just reading them.  The only safe way the API allows
/// password retrieval is by getting them from an unlocked manager.
///
/// This could be rewritten to have a generic identifier and account information type but for the purposes of this demonstration a
/// [HashMap<String, String>] of account usernames to passwords is used.
#[derive(Debug)]
pub struct PasswordManager<State = Locked> {
    master_password: String,
    password_list: HashMap<String, String>,
    state: PhantomData<State>,
}

impl PasswordManager<Locked> {
    /// Attempt to unlock a password manager using the master password.
    ///
    /// Because the locked and unlocked managers are technically different types, this method has to return a
    /// [Result<PasswordManager\<Unlocked>, PasswordManager\<Locked>>].  This has a few benefits:
    /// - It forces the API user to handle the case of an invalid password being entered.
    /// - Since this function moves the password manager, the Err variant gives back the original locked password manager in case of the wrong password.
    pub fn unlock(
        self,
        master_password: impl Into<String>,
    ) -> Result<PasswordManager<Unlocked>, PasswordManager<Locked>> {
        // Accepting an `impl Into<String>` is more flexible for the API caller than just `String` or `&str`.
        let password = master_password.into();
        match password == self.master_password {
            // In the future, if RFC 2528 passes, this could be replaced with `true => Ok(PasswordManager { ..self }),`.
            true => Ok(PasswordManager {
                master_password: self.master_password,
                password_list: self.password_list,
                state: PhantomData,
            }),
            false => Err(self),
        }
    }
}

// Functions only implemented on unlocked password managers.
impl PasswordManager<Unlocked> {
    /// Lock this password manager so that the master password is required to unlock it again.
    pub fn lock(self) -> PasswordManager<Locked> {
        PasswordManager {
            master_password: self.master_password,
            password_list: self.password_list,
            state: PhantomData,
        }
    }

    /// Get a list of the stored accounts and their passwords.
    pub fn get_passwords(&self) -> HashMap<String, String> {
        self.password_list.clone()
    }

    /// Get a single password given the account.
    pub fn get_password(&self, account: &str) -> Option<String> {
        self.password_list.get(account).map(|s| s.to_owned())
    }

    /// Insert a new account and password into the password manager.
    pub fn insert(&mut self, account: impl Into<String>, password: impl Into<String>) {
        self.password_list.insert(account.into(), password.into());
    }
}

/// Denotes that a [PasswordManagerBuilder] hasn't had its master password set yet.
pub struct MissingPassword;
/// Denotes that a [PasswordManagerBuilder] has had its master password set.
pub struct MasterPassword(String);

/// A struct for implementing the builder pattern for the [PasswordManager].
pub struct PasswordManagerBuilder<P = MissingPassword> {
    master_password: P,
    password_list: HashMap<String, String>,
}

impl PasswordManagerBuilder {
    /// Create a new password manager builder with no master password and an empty account list.
    pub fn new() -> Self {
        PasswordManagerBuilder {
            master_password: MissingPassword,
            password_list: HashMap::new(),
        }
    }
}

impl Default for PasswordManagerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Implement `with_account(..)` for password manager builders irrespective of whether the master password is set or not.
impl<P> PasswordManagerBuilder<P> {
    /// Add an account and password to the password manager.
    pub fn with_account(self, account: impl Into<String>, password: impl Into<String>) -> Self {
        let mut new_password_list = self.password_list.clone();
        new_password_list.insert(account.into(), password.into());
        Self {
            password_list: new_password_list,
            ..self
        }
    }
}

// Implement `.with_master_password(..)` only for builders where the master password hasn't been set yet.
// This could be implemented over generic P to be callable multiple times but it only needs to be set once.
impl PasswordManagerBuilder<MissingPassword> {
    /// Set the master password field for this password manager.  If this method is not called on a [PasswordManagerBuilder], the `.build()` method cannot
    /// be called as this would result in an invalid (un-unlockable) password manager.
    pub fn with_master_password(
        self,
        master_password: impl Into<String>,
    ) -> PasswordManagerBuilder<MasterPassword> {
        PasswordManagerBuilder {
            master_password: MasterPassword(master_password.into()),
            password_list: self.password_list,
        }
    }
}

// Implement `.build(..)` only for builders of the MasterPassword type because valid password managers must have a master password set.
impl PasswordManagerBuilder<MasterPassword> {
    /// Build a [PasswordManager] from this builder.
    pub fn build(self) -> PasswordManager {
        PasswordManager {
            master_password: self.master_password.0,
            password_list: self.password_list,
            state: PhantomData,
        }
    }
}
