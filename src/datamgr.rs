use std::sync::{Arc, RwLock, Weak};
use dashmap::DashMap;
use once_cell::sync::OnceCell;

static _MEM_MAP: OnceCell<DashMap<String, Weak<GlobalDataRef>>> = OnceCell::new();

pub fn _mem_map_setup() -> Result<(), ()>{ //normally _names are for unused variable, This function and global variable are almost never meant to be called driectly
    match _MEM_MAP.set(DashMap::new()){
         Ok(_) => Ok(()),
         Err(_) => Err(()),
     }
}

pub struct GlobalDataRef{ //we hold the data in a custom struct to later implemnt the drop trait, this will let us cleanup the dashmap as memory dropped from the system
    data_ref: Vec<Weak<DataEntry>>, //possibly loaded data
    key_ref: String, //reference to the key
}
impl GlobalDataRef{
    pub fn new(_path: &str, _part: Option<&str>) -> Result<Arc<Self>, ()> {
        _MEM_MAP.get().map_or(Err(todo!("Add Map Initialization Error")),|mem_map: &DashMap<String, Weak<GlobalDataRef>>|{ //replace the Error with a proper error
            GlobalDataRef::load_with_retry(mem_map,(0, 3), _path, _part) //Atempts to load the values retrying automatically for certain errors
        })
    }

    fn load_with_retry(mem_map: &DashMap<String, Weak<GlobalDataRef>>, (attempts, retries): (usize, usize), path: &str, part: Option<&str>) -> Result<Arc<Self>, ()>{
        if (attempts == retries){
            Err(todo!("Add Max Retries Error")) //Replace this with a proper error
        } else {mem_map.get(&part.map_or(path.to_string(), |_part| {format!("{}/{}", path, _part)})).map_or_else(||
            {//Code for if a weak reference doesnt exist
            
            todo!()
            },|weak_ref: dashmap::mapref::one::Ref<'_, String, Weak<GlobalDataRef>>| weak_ref.upgrade().map_or_else(||
            {//Code for if weak reference exists but the underlying GlobalDataRef has been dropped (shouldnt happen unless it was dropped right before the upgrade)
                GlobalDataRef::load_with_retry(mem_map, (attempts+1, retries), path, part) //attempt to reload the data (if the ref has dropped it supposed to delete itself from the map using the drop trait, meaning on retry either a new valid entry should exist or no entry should exist)
            }, |arc_ref| Ok(arc_ref))
        )}
    }

    fn clean_key(&mut self){
        _MEM_MAP.get().and_then(|mem_map: &DashMap<String, Weak<GlobalDataRef>>|
            mem_map.remove_if(&self.key_ref, |_, _ref: &Weak<GlobalDataRef>| _ref.upgrade().is_none())
        );
    }
}
impl Drop for GlobalDataRef{
    fn drop(&mut self) {
        self.clean_key();
    }
}
pub struct DataEntry{
    owned_data: Arc<RwLock<[u8]>>,
}
