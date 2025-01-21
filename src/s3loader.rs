use std::sync::{Arc, Weak};
use once_cell::sync::Lazy;
use dashmap::DashMap;


//Holds file data, this data is not owned by the map and can be dropped from memory,
// load_ref function needs to account for this in the case it tries to recieve invalid weak
static MEM_MAP: Lazy<DashMap<String, Weak<[u8]>>> = Lazy::new(|| DashMap::new());

pub fn load_ref<'a>(path: &str, part: Option<&str>) -> Result<Arc<[u8]>, ()> {
    todo!()
}