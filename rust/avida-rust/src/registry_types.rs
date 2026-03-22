//! Newtype IDs for reactions and resources, preventing mixups at the type level.
//!
//! These replace raw `int` indices used throughout the codebase with
//! type-safe wrappers that cannot be accidentally interchanged.

use std::ffi::c_int;

/// Type-safe reaction identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ReactionId(pub c_int);

/// Type-safe resource identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ResourceId(pub c_int);

/// Type-safe task identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct TaskId(pub c_int);

/// Type-safe cell identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct CellId(pub c_int);

/// Type-safe organism identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OrganismId(pub c_int);

/// Type-safe deme identifier.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DemeId(pub c_int);

// FFI constructors for each ID type

#[no_mangle]
pub extern "C" fn avd_reaction_id(raw: c_int) -> ReactionId {
    ReactionId(raw)
}

#[no_mangle]
pub extern "C" fn avd_resource_id(raw: c_int) -> ResourceId {
    ResourceId(raw)
}

#[no_mangle]
pub extern "C" fn avd_task_id(raw: c_int) -> TaskId {
    TaskId(raw)
}

#[no_mangle]
pub extern "C" fn avd_cell_id(raw: c_int) -> CellId {
    CellId(raw)
}

#[no_mangle]
pub extern "C" fn avd_organism_id(raw: c_int) -> OrganismId {
    OrganismId(raw)
}

#[no_mangle]
pub extern "C" fn avd_deme_id(raw: c_int) -> DemeId {
    DemeId(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ids_are_distinct_types() {
        let r = ReactionId(1);
        let s = ResourceId(1);
        // Same raw value but different types — cannot be mixed
        assert_eq!(r.0, s.0);
        // r == s would not compile — that's the point
    }

    #[test]
    fn test_default_is_zero() {
        assert_eq!(ReactionId::default().0, 0);
        assert_eq!(ResourceId::default().0, 0);
        assert_eq!(TaskId::default().0, 0);
    }

    #[test]
    fn test_ffi_constructors() {
        assert_eq!(avd_reaction_id(42).0, 42);
        assert_eq!(avd_resource_id(99).0, 99);
        assert_eq!(avd_task_id(7).0, 7);
        assert_eq!(avd_cell_id(100).0, 100);
    }

    #[test]
    fn test_hash_works() {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(ReactionId(1), "NOT");
        map.insert(ReactionId(2), "NAND");
        assert_eq!(map.get(&ReactionId(1)), Some(&"NOT"));
        assert_eq!(map.get(&ReactionId(3)), None);
    }

    #[test]
    fn test_repr_c_size() {
        assert_eq!(
            std::mem::size_of::<ReactionId>(),
            std::mem::size_of::<c_int>()
        );
        assert_eq!(
            std::mem::size_of::<ResourceId>(),
            std::mem::size_of::<c_int>()
        );
    }
}
