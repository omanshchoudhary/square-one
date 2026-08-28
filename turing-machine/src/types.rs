use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub enum Direction {
    #[serde(rename = "L")]
    Left,
    #[serde(rename = "R")]
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExecutionStatus {
    Running,
    Accepted,
    Rejected,
    HaltUndefined,
    LimitReached,
}
