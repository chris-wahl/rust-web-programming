use std::env;

use rand::prelude::*;

/// Defines the struct to be a User
trait IsUser {
    /// Proclaims the struct is a user
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (bool): `true` is user, `false` if not
    fn is_user() -> bool {
        true
    }
}

/// The struct defining a user
///
/// # Attributes
/// * name (String): the name of the user
/// * age (u8): the age of the user
struct User {
    name: String,
    age: u8,
}

impl IsUser for User {}

/// This function generates a float number between 0 and 10 using provided generator.
///
/// # Arguments
/// * generator (&mut ThreadRng): the random number generator to be used
///
/// # Returns
/// (f64): random number between 0 -> 10
fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();
    return placeholder * 10.0;
}

fn main() {
    let mut rng = rand::thread_rng();
    println!["{}", generate_float(&mut rng)];

    let args: Vec<String> = env::args().collect();
    println!["{:?}", args];

    let path = &args[0];
    if path.contains("/debug/") {
        println!["The development app is running!"];
    } else if path.contains("/release/") {
        println!["The production server is running!"];
    } else {
        panic!["The setting is neither debug or release!"];
    }
}
