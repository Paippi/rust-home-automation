//! Module that holds rule specific functionality.
//!
//! Examples:
//!
//! ```
//! use rust_home_automation::rules::Rule;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     println!("Starting home automation system...");
//!     let my_rule = Rule::new(&"Rule name", || async {
//!
//!         // Add your rule specific functionality here.
//!
//!         res = 1==1;
//!         Ok(res)
//!     });
//!
//!     println!("Monitoring {}", my_rule.name);
//!     loop {
//!         let rule_result = my_rule.poll().await?;
//!         if rule_result == true {
//!             println!("Some message to sent...");
//!         }
//!         // Just some waiting before checking the status again.
//!         thread::sleep(Duration::from_millis(5000));
//!     }
//! }
//! ```

/// Structure to hold a rule.
pub struct Rule<'a, C, F, T>
where
    C: Fn() -> F,
    F: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    pub name: &'a str,
    pub f: C,
}

impl<'a, C, F, T> Rule<'a, C, F, T>
where
    C: Fn() -> F,
    F: std::future::Future<Output = Result<T, Box<dyn std::error::Error>>>,
{
    /// Function that returns the rules output.
    pub async fn poll(&self) -> Result<T, Box<dyn std::error::Error>> {
        (self.f)().await
    }

    /// Construts a new `Rule`.
    pub fn new(name: &'a str, f: C) -> Self {
        Rule { f, name }
    }
}
