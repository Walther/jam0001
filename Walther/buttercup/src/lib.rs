#![deny(clippy::all)]

#[derive(Debug, Clone)]
pub struct Documentation {
    content: String,
}

#[derive(Debug, Clone)]
pub struct Assertion {
    left: String,
    right: String,
}

#[derive(Debug, Clone)]
pub struct Trace {
    content: String,
}

#[derive(Debug, Clone)]
pub enum Comment {
    Documentation(Documentation),
    Assertion(Assertion),
    Trace(Trace),
}

#[cfg(test)]
mod tests {}
