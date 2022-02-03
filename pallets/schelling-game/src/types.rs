use frame_support::{pallet_prelude::*};
use frame_support::sp_std::{vec::Vec, collections::btree_map::BTreeMap};
use scale_info::TypeInfo;


#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SchellingType {
    ProfileApproval{ citizen_id: u128 }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum SumTreeName {
    UniqueIdenfier{ citizen_id: u128, name: Vec<u8>}
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SortitionSumTree {
    pub k: u64,
    pub stack: Vec<u64>,
    pub nodes: Vec<u64>,
    pub ids_to_node_indexes: BTreeMap<u128, u64>, // citizen id, node index
    pub node_indexes_to_ids: BTreeMap<u64, u128>, // node index, citizen id
}





