use ic_stable_memory::{ derive::{ StableType, AsFixedSizeBytes }, collections::SHashMap };

use crate::core::types::IDKey;

// Directory used for converting IDKeys (ICRC/ICP Accounts) to and from a unique u32 referece number.
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Directory {
    pub id_to_ref: SHashMap<IDKey, u64>,
    pub ref_to_id: SHashMap<u64, IDKey>,
    pub next_ref: u64,
}
impl Directory {
    pub fn add_id(&mut self, id_string: String) -> Option<u64> {
        let id_bytes = IDKey::from_string(&id_string);
        match id_bytes {
            Some(key) => {
                let v = self.id_to_ref.get(&key);
                match v {
                    Some(v) => {
                        return Some(v.to_owned());
                    }
                    None => {
                        // not handling result on inserts as
                        // id_bytes match handles 'entry exists already' scenario.
                        self.id_to_ref.insert(key.clone(), self.next_ref).expect("Storage is full");
                        self.ref_to_id.insert(self.next_ref, key).expect("Storage is full");
                        let ret = self.next_ref.clone();
                        self.next_ref += 1_u64;
                        return Some(ret);
                    }
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn get_id(&self, id_ref: &u64) -> Option<String> {
        let v = &self.ref_to_id.get(&id_ref);
        match v {
            Some(v) => {
                let id_string = v.to_string();
                match id_string {
                    Some(id) => {
                        return Some(id);
                    }
                    None => {
                        return None;
                    }
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn get_ref(&self, id_string: &String) -> Option<u64> {
        let opt_key = IDKey::from_string(&id_string);
        match opt_key {
            Some(key) => {
                let v = &self.id_to_ref.get(&key);
                match v {
                    Some(v) => {
                        return Some(**v);
                    }
                    None => {
                        return None;
                    }
                }
            }
            None => {
                return None;
            }
        }
    }

    pub fn get_total_entries(&self) -> u64 {
        let res = &self.next_ref;
        return res.to_owned();
    }
}
