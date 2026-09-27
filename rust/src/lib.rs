//! Identify the Cameroonian mobile operator (MTN, Orange, Nexttel, Camtel) that issued a
//! phone number. See `../spec/README.md` for the input rules and prefix table.

use std::fmt;

/// A Cameroonian mobile network operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Mtn,
    Orange,
    Nexttel,
    Camtel,
}

impl Operator {
    /// The lowercase operator name used in the spec and the PHP reference implementation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Operator::Mtn => "mtn",
            Operator::Orange => "orange",
            Operator::Nexttel => "nexttel",
            Operator::Camtel => "camtel",
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The prefix ranges assigned to each operator. A prefix names the operator that issued the
/// number; portability lets a subscriber keep the number on another network.
pub const OPERATOR_PREFIXES: &[(Operator, &[&str])] = &[
    (
        Operator::Mtn,
        &[
            "67", "650", "651", "652", "653", "654", "680", "681", "682", "683",
        ],
    ),
    (
        Operator::Orange,
        &[
            "69", "640", "641", "642", "655", "656", "657", "658", "659", "686", "687", "688",
            "689",
        ],
    ),
    (Operator::Nexttel, &["66", "684", "685"]),
    (Operator::Camtel, &["62", "222", "233", "242", "243"]),
];

/// Returns the operator that issued `tel`, or `None` if it isn't a valid Cameroonian number.
pub fn check(tel: &str) -> Option<Operator> {
    let number = national_number(tel)?;
    OPERATOR_PREFIXES
        .iter()
        .find(|(_, prefixes)| prefixes.iter().any(|prefix| number.starts_with(prefix)))
        .map(|(operator, _)| *operator)
}

pub fn is_mtn(tel: &str) -> bool {
    check(tel) == Some(Operator::Mtn)
}

pub fn is_orange(tel: &str) -> bool {
    check(tel) == Some(Operator::Orange)
}

pub fn is_nexttel(tel: &str) -> bool {
    check(tel) == Some(Operator::Nexttel)
}

pub fn is_camtel(tel: &str) -> bool {
    check(tel) == Some(Operator::Camtel)
}

/// Strips whitespace and the optional `237`/`+237`/`00237` country code prefix, returning the
/// 9-digit national number. `None` if what's left isn't exactly 9 digits.
fn national_number(tel: &str) -> Option<String> {
    let chars: Vec<char> = tel.chars().filter(|c| !c.is_whitespace()).collect();
    if chars.len() < 9 {
        return None;
    }

    let split = chars.len() - 9;
    let digits: String = chars[split..].iter().collect();
    if !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    let prefix: String = chars[..split].iter().collect();
    match prefix.as_str() {
        "" | "237" | "+237" | "00237" => Some(digits),
        _ => None,
    }
}
