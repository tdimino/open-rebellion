//! Serde adapter for the world's slotmaps.
//!
//! `slotmap`'s own serde form keeps every slot and its version, but it
//! rebuilds the free list in descending slot order, whatever order the
//! removals left it in. A loaded world would then give a new object a
//! different slot than the uninterrupted one, and anything that walks a map
//! in slot order (the AI does) plays on differently. This adapter writes the
//! free list's pop order next to the slots and, on load, rebuilds the map so
//! that new inserts reuse the same slots with the same versions.
//!
//! The pop order is read by inserting into a clone until a fresh slot
//! appears; a fresh slot has version 1 and a reused one at least 3, so the
//! key versions also give every vacant slot's version. The rebuild creates
//! the slots in a new map, raises each to its saved version by remove/insert
//! cycles (each adds 2), then frees the vacant ones in reverse pop order.
//! Both need a value to insert: a live one when the map has any, else
//! `V::default()`. The placeholder is only ever inserted into a clone or a
//! map under rebuild and removed again, so no caller sees it.

use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use slotmap::{Key, SlotMap};

/// Bound on remove/insert cycles spent raising slot versions, so a corrupt
/// save cannot stall a load. port: persistence mechanism with no original
/// counterpart (the original saves no arena handles).
const MAX_VERSION_CYCLES: u64 = 1 << 24;

#[derive(Serialize)]
#[serde(bound(serialize = "K: Key, V: Serialize"))]
struct Written<'a, K: Key, V> {
    slots: &'a SlotMap<K, V>,
    free: Vec<u32>,
}

#[derive(Deserialize)]
#[serde(bound(deserialize = "K: Key, V: Deserialize<'de>"))]
struct Read<K: Key, V> {
    slots: SlotMap<K, V>,
    free: Vec<u32>,
}

/// Index and version of a slotmap key. port: persistence mechanism with no
/// original counterpart.
fn slot_of<K: Key>(key: K) -> (u32, u32) {
    let ffi = key.data().as_ffi();
    (ffi as u32, (ffi >> 32) as u32)
}

/// A value to insert while probing or rebuilding: any live one, else the
/// default. port: persistence mechanism with no original counterpart.
fn spare_of<K: Key, V: Clone + Default>(map: &SlotMap<K, V>) -> V {
    map.iter()
        .next()
        .map_or_else(V::default, |(_, value)| value.clone())
}

/// The vacant slots as `(index, version)` in the order inserts will take
/// them.
fn vacant_in_pop_order<K: Key, V: Clone + Default>(map: &SlotMap<K, V>) -> Vec<(u32, u32)> {
    let spare = spare_of(map);
    let mut probe = map.clone();
    let mut vacant = Vec::new();
    loop {
        let (index, version) = slot_of(probe.insert(spare.clone()));
        if version == 1 {
            return vacant;
        }
        vacant.push((index, version - 1));
    }
}

pub fn serialize<K, V, S>(map: &SlotMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
where
    K: Key,
    V: Serialize + Clone + Default,
    S: Serializer,
{
    let free = vacant_in_pop_order(map)
        .into_iter()
        .map(|(index, _)| index)
        .collect();
    Written { slots: map, free }.serialize(serializer)
}

pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<SlotMap<K, V>, D::Error>
where
    K: Key,
    V: Deserialize<'de> + Clone + Default,
    D: Deserializer<'de>,
{
    let Read { slots, free } = Read::deserialize(deserializer)?;
    reorder(slots, &free).map_err(serde::de::Error::custom)
}

/// Rebuilds `slots` so that its vacant slots are taken in `free` order.
/// port: persistence mechanism with no original counterpart.
fn reorder<K: Key, V: Clone + Default>(
    slots: SlotMap<K, V>,
    free: &[u32],
) -> Result<SlotMap<K, V>, String> {
    let vacant = vacant_in_pop_order(&slots);
    if vacant
        .iter()
        .map(|&(index, _)| index)
        .eq(free.iter().copied())
    {
        return Ok(slots);
    }
    let spare = spare_of(&slots);

    let mut versions: BTreeMap<u32, u32> = vacant.iter().copied().collect();
    let mut values: BTreeMap<u32, V> = BTreeMap::new();
    for (key, value) in slots {
        let (index, version) = slot_of(key);
        versions.insert(index, version);
        values.insert(index, value);
    }
    let mut saved = free.to_vec();
    saved.sort_unstable();
    let vacant_sorted: Vec<u32> = versions
        .keys()
        .copied()
        .filter(|index| !values.contains_key(index))
        .collect();
    if saved != vacant_sorted || versions.keys().copied().ne(1..=versions.len() as u32) {
        return Err("the saved free list does not match the vacant slots".into());
    }

    let mut rebuilt = SlotMap::<K, V>::with_key();
    let mut keys: BTreeMap<u32, K> = BTreeMap::new();
    for &index in versions.keys() {
        let value = values.remove(&index).unwrap_or_else(|| spare.clone());
        let key = rebuilt.insert(value);
        if slot_of(key).0 != index {
            return Err("a rebuilt slot landed at the wrong index".into());
        }
        keys.insert(index, key);
    }
    let mut cycles_left = MAX_VERSION_CYCLES;
    for (&index, &version) in &versions {
        let occupied = !free.contains(&index);
        let cycles = u64::from(if occupied {
            (version - 1) / 2
        } else {
            version / 2 - 1
        });
        cycles_left = cycles_left
            .checked_sub(cycles)
            .ok_or("a slot version is implausibly large")?;
        let mut key = keys[&index];
        for _ in 0..cycles {
            let value = rebuilt.remove(key).ok_or("a rebuilt slot was missing")?;
            key = rebuilt.insert(value);
        }
        keys.insert(index, key);
    }
    for index in free.iter().rev() {
        rebuilt.remove(keys[index]);
    }
    if !vacant_in_pop_order(&rebuilt)
        .into_iter()
        .eq(free.iter().map(|index| (*index, versions[index])))
    {
        return Err("the rebuilt free list differs from the saved one".into());
    }
    Ok(rebuilt)
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use slotmap::{new_key_type, SlotMap};

    new_key_type! {
        struct TestKey;
    }

    #[derive(Serialize, Deserialize)]
    struct Saved(#[serde(with = "super")] SlotMap<TestKey, u32>);

    // port: persistence mechanism; a load reproduces the uninterrupted run
    // only if new objects take the same slots (or-ahj2).
    #[test]
    fn a_loaded_map_reuses_freed_slots_in_the_order_the_removals_left() {
        let mut map = SlotMap::<TestKey, u32>::with_key();
        let keys: Vec<TestKey> = (0..5).map(|value| map.insert(value)).collect();
        for index in [1, 3, 0] {
            map.remove(keys[index]);
        }
        let saved = serde_json::to_string(&Saved(map.clone())).unwrap();
        let Saved(mut loaded) = serde_json::from_str(&saved).unwrap();
        for value in 10..14 {
            assert_eq!(loaded.insert(value), map.insert(value), "insert {value}");
        }
    }
}
