use crate::Observation;

/// Owned observations in exactly the order supplied by the caller.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Batch {
    observations: Vec<Observation>,
}

impl Batch {
    pub fn new(observations: Vec<Observation>) -> Self {
        Self { observations }
    }

    pub fn len(&self) -> usize {
        self.observations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.observations.is_empty()
    }

    pub fn as_slice(&self) -> &[Observation] {
        &self.observations
    }

    pub fn into_vec(self) -> Vec<Observation> {
        self.observations
    }
}

impl IntoIterator for Batch {
    type Item = Observation;
    type IntoIter = std::vec::IntoIter<Observation>;

    fn into_iter(self) -> Self::IntoIter {
        self.observations.into_iter()
    }
}
