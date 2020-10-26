use bincode::Error as BincodeError;
use cuckoofilter::{self, CuckooFilter, ExportedCuckooFilter};
use std::convert::From;

use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;

pub type Filters<T> = Vec<(T, CuckooFilter<DefaultHasher>)>;
type ExportedFilters<T> = Vec<(T, ExportedCuckooFilter)>;

pub struct Storage<T> {
    pub filters: Filters<T>,
}

impl<T> From<Filters<T>> for Storage<T> {
    fn from(filters: Filters<T>) -> Self {
        Storage { filters }
    }
}

pub trait Score {
    fn score<A: AsRef<str>, I: IntoIterator<Item = A>>(&self, terms: I) -> u32;
}

// the score is the number of terms from the query that are contained in the
// current filter
impl Score for CuckooFilter<DefaultHasher> {
    fn score<A: AsRef<str>, I: IntoIterator<Item = A>>(&self, terms: I) -> u32 {
        terms
            .into_iter()
            .filter(|term| self.contains(term.as_ref()))
            .count() as u32
    }
}

impl<'a, T> Storage<T>
where
    T: Serialize + Deserialize<'a> + Clone,
{
    pub fn to_bytes(&self) -> Result<Vec<u8>, BincodeError> {
        let encoded: Vec<u8> = bincode::serialize(&self.dehydrate())?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, BincodeError> {
        let decoded: ExportedFilters<T> = bincode::deserialize(bytes)?;
        Ok(Storage {
            filters: Storage::hydrate(decoded),
        })
    }

    fn dehydrate(&self) -> ExportedFilters<T> {
        self.filters
            .iter()
            .map(|(key, filter)| (key.clone(), filter.export()))
            .collect()
    }

    fn hydrate(exported_filters: ExportedFilters<T>) -> Filters<T> {
        exported_filters
            .into_iter()
            .map(|(key, exported)| (key, CuckooFilter::<DefaultHasher>::from(exported)))
            .collect()
    }
}
