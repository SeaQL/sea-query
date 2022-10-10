pub use string::*;
pub use tuple::*;
pub use types::*;

mod string;
mod tuple;
mod types;

#[derive(Debug, Clone)]
pub struct Values(pub Vec<Value>);

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }
}

impl IntoIterator for Values {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
