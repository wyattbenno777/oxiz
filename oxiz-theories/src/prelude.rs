//! Prelude for no_std / std compatibility.

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use alloc::{
    boxed::Box,
    collections::{BTreeMap, BinaryHeap, VecDeque},
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use portable_atomic_util::Arc;

#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use hashbrown::{HashMap, HashSet, hash_map};

#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque, hash_map};

#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use std::sync::Arc;

#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use rustc_hash::{FxHashMap, FxHashSet};

#[cfg(not(feature = "std"))]
pub(crate) type FxHashMap<K, V> =
    hashbrown::HashMap<K, V, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;
#[cfg(not(feature = "std"))]
pub(crate) type FxHashSet<K> =
    hashbrown::HashSet<K, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

#[cfg(not(feature = "std"))]
#[allow(unused_macros)]
macro_rules! println {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use println;

#[cfg(not(feature = "std"))]
#[allow(unused_macros)]
macro_rules! eprintln {
    ($($arg:tt)*) => {};
}
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
pub(crate) use eprintln;

// Override eprintln! as a no-op in std mode too.
// ZeroOS (Jolt zkVM guest runtime) does not support the stderr write syscall —
// any attempt to write to stderr causes a trap and guest panic.
// This override applies to all oxiz crates that use this prelude.
#[cfg(feature = "std")]
#[allow(unused_macros)]
macro_rules! eprintln {
    ($($arg:tt)*) => {
        // no-op: stderr not supported in ZeroOS guest
        let _ = format!($($arg)*);
    };
}
#[cfg(feature = "std")]
#[allow(unused_imports)]
pub(crate) use eprintln;