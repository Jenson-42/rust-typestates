# Rust Type States Demo

This is a demonstration of how Rust's type system can be leveraged to make robust APIs with important compile-time checks.
The documentation in this crate is written within the modules themselves, Start by looking into into "password_manager.rs" to begin.

This crate implements a password manager API.  In most languages, you'd be forced to add an `isUnlocked` field to your `PasswordManager` class and call it a day. Maybe, if you wanted to be fancy, you'd return an `Option<Password>` to indicate whether the manager was unlocked or not.  Or, you could have an `UnlockedPasswordManager` class and a `LockedPasswordManager` class where you'd have to manually ensure that any common fields or functionality were updated on both.  Using generic types, Rust automatically creates an `Unlocked` and `Locked` variant of the `PasswordManager` struct at compile time, preserving common functionality while also allowing separate methods to be defined for them. 