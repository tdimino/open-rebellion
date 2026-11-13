//! Deterministic human-readable serde adapters for unordered collections.
//!
//! Bincode keeps the historical map/set wire shape. Human-readable formats
//! use sorted entry sequences so typed slotmap keys are representable in JSON
//! and canonical state fingerprints do not depend on `HashMap` random seeds.

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn serialize_ordered_by_key<S, T, K>(
    values: impl Iterator<Item = T>,
    key: impl Fn(&T) -> &K,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
    K: Serialize + ?Sized,
{
    let mut ordered = values
        .map(|value| {
            serde_json::to_vec(key(&value))
                .map(|bytes| (bytes, value))
                .map_err(serde::ser::Error::custom)
        })
        .collect::<Result<Vec<_>, S::Error>>()?;
    ordered.sort_by(|left, right| left.0.cmp(&right.0));
    ordered
        .into_iter()
        .map(|(_, value)| value)
        .collect::<Vec<_>>()
        .serialize(serializer)
}

pub fn serialize_hash_map<S, K, V, H>(
    values: &HashMap<K, V, H>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    K: Serialize,
    V: Serialize,
    H: BuildHasher,
{
    if !serializer.is_human_readable() {
        return values.serialize(serializer);
    }

    serialize_ordered_by_key(values.iter(), |(key, _)| key, serializer)
}

pub fn deserialize_hash_map<'de, D, K, V>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: DeserializeOwned + Eq + Hash,
    V: DeserializeOwned,
{
    if deserializer.is_human_readable() {
        return Vec::<(K, V)>::deserialize(deserializer)
            .map(|entries| entries.into_iter().collect());
    }
    HashMap::deserialize(deserializer)
}

pub fn serialize_hash_set<S, T, H>(values: &HashSet<T, H>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
    H: BuildHasher,
{
    if !serializer.is_human_readable() {
        return values.serialize(serializer);
    }

    serialize_ordered_by_key(values.iter(), |value| *value, serializer)
}

pub fn deserialize_hash_set<'de, D, T>(deserializer: D) -> Result<HashSet<T>, D::Error>
where
    D: Deserializer<'de>,
    T: DeserializeOwned + Eq + Hash,
{
    if deserializer.is_human_readable() {
        return Vec::<T>::deserialize(deserializer).map(|entries| entries.into_iter().collect());
    }
    HashSet::deserialize(deserializer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct Collections {
        #[serde(
            serialize_with = "serialize_hash_map",
            deserialize_with = "deserialize_hash_map"
        )]
        map: HashMap<(u32, u32), String>,
        #[serde(
            serialize_with = "serialize_hash_set",
            deserialize_with = "deserialize_hash_set"
        )]
        set: HashSet<(u32, u32)>,
    }

    #[test]
    fn json_supports_typed_keys_and_stable_order() {
        let mut first = Collections {
            map: HashMap::new(),
            set: HashSet::new(),
        };
        first.map.insert((2, 1), "second".into());
        first.map.insert((1, 2), "first".into());
        first.set.insert((4, 3));
        first.set.insert((3, 4));

        let encoded = serde_json::to_string(&first).unwrap();
        let decoded: Collections = serde_json::from_str(&encoded).unwrap();

        assert_eq!(decoded.map, first.map);
        assert_eq!(decoded.set, first.set);
        assert_eq!(encoded, serde_json::to_string(&first).unwrap());
    }
}
