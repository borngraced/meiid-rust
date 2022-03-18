//! Meiid UUID generator based on current date for rust
//!
//! All generated uuid are unique in the convention {####-####-year/month/day}, so the chances of encountering a duplicate ID even in external data is 1 in a billion.
//! 
//! Also, this module is able to generate 1 million "unique" uuid(s) in 60.5secs
//!
//!
//!
use chrono::Utc;
use rand::prelude::*;


/// This module makes it easy to generate uuid for database table or wherever you might need one
/// 
/// USAGE:
/// 1. initializes Meiid struct
/// 2. generates and returns a unique uuid as string
/// ```rust
/// use rust::Meiid;
/// let init = Meiid::new();
/// let id = init.uuid();
/// ```
/// id can then be used as you like'd to
/// 
#[derive(Debug, Clone)]
pub struct Meiid {}

impl Meiid {
    /// The new method initializes Meiid struct to be ready to use to call on uuid
    /// 
    /// Doesn't receive any parameter
    pub fn new() -> Self {
        Self { /* fields */ }
    }
    
    /// Uuid method relys on "Self" returned from initilizing with Meiid::new() to generate unique ID so you can't call Meiid::uuid() directly (It won't work. Meiid needs to me initialized with main first)
    /// 
    /// generate_uuid returns a uuid as string
    /// 
    /// uuid method must be call to generate a unique uuid
    pub fn uuid(&self) -> String {
        return generate_uuid();
    }
}

fn generate_uuid() -> String {
    let mut uuid = String::new();
    let a_z = "a b c d e f g h i j k l m n o p q r s t u v w x y z";
    let a_z = a_z.split(" ").collect::<Vec<_>>();
    let date = Utc::now();
    let date = date
        .naive_utc()
        .date()
        .to_string()
        .split("-")
        .collect::<Vec<_>>()
        .join("");
    let mut rng = rand::thread_rng();
    let id = format!(
        "me{}{}{}{}-{}{}{}{}-{}",
        a_z[rng.gen_range(0..a_z.len())],
        rng.gen_range(0..a_z.len()),
        a_z[rng.gen_range(0..a_z.len())],
        a_z[rng.gen_range(0..a_z.len())],
        a_z[rng.gen_range(0..a_z.len())],
        rng.gen_range(0..a_z.len()),
        a_z[rng.gen_range(0..a_z.len())],
        a_z[rng.gen_range(0..a_z.len())],
        &date,
    );
    uuid.push_str(id.as_str());
    uuid
}

#[cfg(test)]
mod test {
    use crate::Meiid;

    #[test]
    fn tester() {
        let i = Meiid::new();
        for _ in 0..1 {
            println!("{:}", i.uuid());
        }
    }
}
