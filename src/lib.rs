//! Crate for rust home automation!
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
pub mod rules;
