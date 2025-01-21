use std::{error::Error, fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Profile {
    #[serde(rename = "test")]
    Test,
    #[serde(rename = "development")]
    Dev,
    #[serde(rename = "production")]
    Prod,
}

impl Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Profile::Test => write!(f, "test"),
            Profile::Dev => write!(f, "development"),
            Profile::Prod => write!(f, "production"),
        }
    }
}

#[derive(Debug)]
pub struct ParseProfileErr;

impl Display for ParseProfileErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse profile enmu  type error")
    }
}

impl Error for ParseProfileErr {
    // add code here
}

impl FromStr for Profile {
    type Err = ParseProfileErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "test" => Ok(Self::Test),
            "development" => Ok(Self::Dev),
            "production" => Ok(Self::Prod),
            _ => Err(ParseProfileErr),
        }
    }
}

#[test]
fn profile_test() {
    let profile = Profile::from_str("test").unwrap();
    println!("p ={:?}", profile);
    assert_eq!(1, 2)
}
