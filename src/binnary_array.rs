// #![allow(unused)]
use std::{
    cell::Cell,
    ops::{Index, IndexMut},
};

pub struct BinaryArray {
    data: Cell<Vec<u64>>,
    length: usize,

    /// used instead of returing actual bit
    bool_refence: bool, 
    previous_index: usize,
    update_sieve: Cell<bool>,
}

impl BinaryArray {
    pub fn new(lenght: usize, default_value: bool) -> BinaryArray {
        BinaryArray {
            data: Cell::new(vec![
                if default_value { u64::MAX } else { 0 };
                (lenght + 63) / 64
            ]),
            length: lenght,
            bool_refence: default_value,
            previous_index: 0,
            update_sieve: Cell::new(false),
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    /// Sets value at given index  
    /// safer then using IndexMut trait
    #[allow(unused)]
    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.length {
            panic!("Index out of bounds");
        }
        let chunk = index / 64;
        let index_in_chunk = 63 - index % 64;
        if value {
            self.data.get_mut()[chunk] |= 1 << index_in_chunk;
        } else {
            self.data.get_mut()[chunk] &= !(1 << index_in_chunk);
        }
    }

    /// commits changed what are created due to IndexMut trait
    fn commit_change(&self) {
        if self.update_sieve.get() {
            unsafe {
                let update_sieve_ref = &mut *self.update_sieve.as_ptr();
                *update_sieve_ref = false;

                let sieve_refence = &mut *self.data.as_ptr();
                let chunk = self.previous_index / 64;
                let index_in_chunk = 63 - self.previous_index % 64;
                if self.bool_refence {
                    sieve_refence[chunk] |= 1 << index_in_chunk;
                } else {
                    sieve_refence[chunk] &= !(1 << index_in_chunk);
                }
            }
        }
    }
}

impl Index<usize> for BinaryArray {
    type Output = bool;
    fn index(&self, index: usize) -> &bool {
        self.commit_change(); // ensures that previus change is done

        if index >= self.length {
            panic!("Index out of bounds");
        }

        if (unsafe { &(&(*self.data.as_ptr()))[index / 64] }) & (1 << (63 - index % 64)) != 0 {
            &true
        } else {
            &false
        }
    }
}

/// sets value at given index 
/// #WARNING  
/// there migth be bugs while using asynchronous code
impl IndexMut<usize> for BinaryArray {
    fn index_mut(&mut self, index: usize) -> &mut bool {
        self.commit_change();

        self.previous_index = index;
        self.update_sieve = Cell::new(true);
        &mut self.bool_refence
    }
}

impl IntoIterator for BinaryArray {
    type Item = bool;
    type IntoIter = BinaryArrayIterator;

    fn into_iter(self) -> Self::IntoIter {
        // tworzy iterator dla BinaryArray
        BinaryArrayIterator {
            binary_array: self,
            index: 0,
        }
    }
}

pub struct BinaryArrayIterator {
    binary_array: BinaryArray,
    index: usize,
}

impl Iterator for BinaryArrayIterator {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.binary_array.len() {
            let value = self.binary_array[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
