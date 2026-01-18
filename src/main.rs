use rand::Rng;

/// This trait defines the struct to be a user.
trait IsUser {
    /// This function proclaims that the struct is a user.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (bool) true if user, false if not
    fn is_user() -> bool {
        return true
    }
}
/// This struct defines a user
///
/// # Attributes
/// * name (String): the name of the user
/// * age (i8): the age of the user
struct User {
    name: String,
    age: i8
}

fn generate_float(rng: &mut impl Rng) -> f64 {
    rng.random_range(0.0..50.0)
}

fn main() {
    let random_number = generate_float(&mut rand::rng());
    println!("{}", random_number);
}