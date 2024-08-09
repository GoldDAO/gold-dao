use candid::CandidType;
use ic_stable_memory::{derive::StableType, AsFixedSizeBytes};
use serde::{Serialize, Deserialize};


// [][] --- Types for Utils --- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct MemoryData {
   pub memory: u64,
   pub heap_memory: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}

// ID Key is limited to 135 bytes (max 134 input string and ':' at the end) 
#[derive(CandidType, Deserialize, StableType, Hash, Eq, PartialEq, Clone, Debug)]
pub struct IDKey(pub Vec<u8>);
impl AsFixedSizeBytes for IDKey {
    const SIZE: usize = 135;
    type Buf =  Vec<u8>; // use for generics  
    
    fn as_fixed_size_bytes(&self, buf: &mut [u8]) {
        let key_bytes = self.0.as_slice();
        buf[0] =  key_bytes.len() as u8;
        buf[1..(1 + key_bytes.len())].copy_from_slice(key_bytes);
    }
    
    fn from_fixed_size_bytes(buf: &[u8]) -> Self {
        let key_len = buf[0] as usize;
        let key: &[u8] = &buf[1..(1 + key_len)];
        return IDKey(key.try_into().unwrap());
    }
}
impl Default for IDKey {
    fn default() -> Self {
        IDKey(Vec::new()) 
    }
}
impl IDKey {
    // MAX 135 Bytes length!! 
    pub fn from_string(input: &String) -> Option<IDKey>{
        if input.len() > 134 {return None} // len in bytes not chars
        let s = format!("{}:",input); // show end of string with :
        let bytes: Vec<u8> = s.to_owned().into_bytes(); 
        return Some(IDKey(bytes));
    }
    pub fn from_str(input: &str) -> Option<IDKey>{
        if input.len() > 134 {return None} // len in bytes not chars
        let s = format!("{}:",input); // show end of string with :
        let bytes: Vec<u8> = s.to_owned().into_bytes(); 
        return Some(IDKey(bytes));
    }
    pub fn to_string(&self) -> Option<String> {
        if let Some(pos) = self.0.iter().position(|&a| a == b':') {
            let id_slice = &self.0[..pos];
            let res_string = std::str::from_utf8(id_slice).map(|s| s.to_string());
            match res_string {
                Ok(output) => {
                    return Some(output);
                }, 
                Err(_error) => {
                    return None;
                }
            }
        } else {
            return None;
        }
    }
}

