use super::doubly_list::DoublyList;
use super::hash_table::{DoubleHashTable, HashTable};
use super::hash::Hashable;
use super::node::DoublyNode;
use std::ptr;

#[derive(PartialEq)]
pub struct KVPair<K, V> {
    pub key: K,
    pub value: V,
}

pub struct LRUCache<K, V> {
    pub capacity: usize,
    pub size: usize,
    pub list: DoublyList<KVPair<K, V>>,
    pub map: DoubleHashTable<K, Option<*mut DoublyNode<KVPair<K, V>>>>,
}

impl<K: Hashable + Default + Clone + PartialEq, V: Clone + PartialEq> LRUCache<K, V> {
    pub fn new(cap: usize) -> Self {
        LRUCache {
            capacity: cap,
            size: 0,
            list: DoublyList::new(),
            map: DoubleHashTable::withCapacity(cap),
        }
    }

    fn unlink(&mut self, n: *mut DoublyNode<KVPair<K, V>>) {
        unsafe {
            if !(*n).prev.is_null() {
                (*(*n).prev).next = (*n).next;
            } else {
                self.list.head = (*n).next;
            }
            if !(*n).next.is_null() {
                (*(*n).next).prev = (*n).prev;
            } else {
                self.list.tail = (*n).prev;
            }
        }
    }

    fn pushFront(&mut self, n: *mut DoublyNode<KVPair<K, V>>) {
        unsafe {
            (*n).prev = ptr::null_mut();
            (*n).next = self.list.head;
            if !self.list.head.is_null() {
                (*self.list.head).prev = n;
            } else {
                self.list.tail = n;
            }
            self.list.head = n;
        }
    }

    pub fn set(&mut self, key: K, value: V) {
        if self.capacity == 0 { return; }

        if self.map.contains(&key) {
            let n = self.map.get(&key).unwrap();
            unsafe { (*n).data.value = value; }
            self.unlink(n);
            self.pushFront(n);
        } else {
            if self.size == self.capacity {
                let old = self.list.tail;
                unsafe {
                    let oldKey = (*old).data.key.clone();
                    self.map.remove(&oldKey);
                }
                self.unlink(old);
                unsafe { drop(Box::from_raw(old)); }
            } else {
                self.size += 1;
            }
            let node = DoublyNode::new(
                KVPair { key: key.clone(), value },
                ptr::null_mut(),
                ptr::null_mut(),
            );
            let n = Box::into_raw(Box::new(node));
            self.pushFront(n);
            self.map.insert(key, Some(n));
        }
    }

    pub fn get(&mut self, key: &K) -> *mut V {
        if !self.map.contains(key) {
            return ptr::null_mut();
        }
        let n = self.map.get(key).unwrap();
        self.unlink(n);
        self.pushFront(n);
        unsafe { &mut (*n).data.value as *mut V }
    }
}