//! This build script checks for incompatible feature sets.
//! Allowed feature sets:
//! - `softatui`
//! - `crossterm`
//! - `hybrid-contexts`
//! Note that the list is exhaustive.

use std::collections::{HashMap, HashSet};

fn main() {
    // let is_rust_analyzer = std::env::var("RUST_ANALYZER_EXT").is_ok();
    // let mut valid_feature_sets = vec![
    //     {
    //         let mut set = HashSet::new();
    //         set.insert("softatui".to_string());
    //         set
    //     },
    //     {
    //         let mut set = HashSet::new();
    //         set.insert("crossterm".to_string());
    //         set
    //     },
    //     {
    //         let mut set = HashSet::new();
    //         set.insert("hybrid-contexts".to_string());
    //         set
    //     },
    // ];

    // if is_rust_analyzer {
    //     // Add no features as a valid feature so that
    //     // you get full RA coverage on cross-platform development
    //     valid_feature_sets.push(HashSet::new());
    // }

    // let enabled_features: HashSet<String> = std::env::vars()
    //     .filter_map(|(key, val)| {
    //         if key.starts_with("CARGO_FEATURE_") && val == "1" {
    //             Some(key.trim_start_matches("CARGO_FEATURE_").to_string().to_lowercase())
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // let is_set_valid = valid_feature_sets.iter().any(|set| set == &enabled_features);

    // if !is_set_valid {
    //     panic!(
    //         "Incompatible feature set enabled. Allowed feature sets are:\n\
    //         - only `softatui`\n\
    //         - only `crossterm`\n\
    //         - only `hybrid-contexts`\n\n\
    //         Enabled features: {:?}",
    //         enabled_features
    //     );
    // }
}
