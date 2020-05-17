use super::shared::{AddressDebugInfo, SymbolicationResult};
use std::collections::HashMap;
use std::ops::Deref;

#[repr(C)]
pub struct CompactSymbolTable {
    pub addr: Vec<u32>,
    pub index: Vec<u32>,
    pub buffer: Vec<u8>,
}

impl CompactSymbolTable {
    pub fn new() -> Self {
        Self {
            addr: Vec::new(),
            index: Vec::new(),
            buffer: Vec::new(),
        }
    }

    fn add_name(&mut self, name: &str) {
        self.buffer.extend_from_slice(name.as_bytes());
    }
}

impl SymbolicationResult for CompactSymbolTable {
    fn from_full_map<T: Deref<Target = str>>(map: HashMap<u32, T>, _addresses: &[u32]) -> Self {
        let mut table = Self::new();
        let mut entries: Vec<_> = map.into_iter().collect();
        entries.sort_by_key(|&(addr, _)| addr);
        for (addr, name) in entries {
            table.addr.push(addr);
            table.index.push(table.buffer.len() as u32);
            table.add_name(&name);
        }
        table.index.push(table.buffer.len() as u32);
        table
    }

    fn from_map_with_addresses<S>(
        _map: HashMap<u32, S>,
        _addresses: &[u32],
        _total_symbol_count: u32,
    ) -> Self
    where
        S: Deref<Target = str>,
    {
        panic!("Should not be called")
    }

    fn wants_address_debug_info() -> bool {
        false
    }

    fn wants_full_map() -> bool {
        true
    }

    fn add_address_debug_info(&mut self, _address: u32, _info: AddressDebugInfo) {
        panic!("Should not be called")
    }
}
