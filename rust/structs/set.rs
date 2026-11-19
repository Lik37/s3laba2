use super::forward_list::ForwardList;
use super::hash::Hashable;
use std::alloc::{alloc, dealloc, Layout};
use std::fmt::Display;
use std::io::Write;
use std::ptr;

pub struct Set<T> {
    pub capacity: usize,
    pub data: *mut ForwardList<T>,
    pub size: usize,
}

fn allocBuckets<T>(cap: usize) -> *mut ForwardList<T> {
    if cap == 0 { return ptr::null_mut(); }
    let layout = Layout::array::<ForwardList<T>>(cap).unwrap();
    let p = unsafe { alloc(layout) as *mut ForwardList<T> };
    if p.is_null() { panic!("allocation failed"); }
    for i in 0..cap {
        unsafe {
            p.add(i).write(ForwardList {
                head: ptr::null_mut(),
                tail: ptr::null_mut(),
            });
        }
    }
    p
}

fn deallocBuckets<T>(p: *mut ForwardList<T>, cap: usize) {
    if cap == 0 || p.is_null() { return; }
    for i in 0..cap {
        unsafe { ptr::drop_in_place(p.add(i)); }
    }
    let layout = Layout::array::<ForwardList<T>>(cap).unwrap();
    unsafe { dealloc(p as *mut u8, layout); }
}

impl<T> Set<T> {
    pub fn new() -> Self {
        Set { capacity: 8, data: allocBuckets(8), size: 0 }
    }
    pub fn isEmpty(&self) -> bool { self.size == 0 }
}

impl<T: PartialEq + Hashable + Clone> Set<T> {
    fn resize(&mut self) {
        let old_cap = self.capacity;
        let old_data = self.data;

        self.capacity *= 2;
        self.data = allocBuckets(self.capacity);
        self.size = 0;

        for i in 0..old_cap {
            let mut cur = unsafe { (*old_data.add(i)).head };
            while !cur.is_null() {
                unsafe {
                    self.add(&(*cur).data);
                    cur = (*cur).next;
                }
            }
        }
        deallocBuckets(old_data, old_cap);
    }

    pub fn add(&mut self, value: &T) -> bool {
        if self.contains(value) { return false; }
        if self.size + 1 > self.capacity { self.resize(); }
        let idx = value.hash(self.capacity);
        unsafe { (*self.data.add(idx)).pushBack(value.clone()); }
        self.size += 1;
        true
    }

    pub fn contains(&self, value: &T) -> bool {
        let idx = value.hash(self.capacity);
        unsafe { !(*self.data.add(idx)).find(value).is_null() }
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let idx = value.hash(self.capacity);
        let removed = unsafe { (*self.data.add(idx)).deleteValue(value) };
        if removed { self.size -= 1; }
        removed
    }

    pub fn unite(&self, other: &Set<T>) -> Set<T> {
        let mut result = Set::new();
        deallocBuckets(result.data, result.capacity);
        result.capacity = self.capacity;
        result.data = allocBuckets(self.capacity);
        result.size = 0;
        for i in 0..self.capacity {
            let mut cur = unsafe { (*self.data.add(i)).head };
            while !cur.is_null() {
                unsafe {
                    let data = (*cur).data.clone();
                    let idx = data.hash(result.capacity);
                    (*result.data.add(idx)).pushBack(data);
                    result.size += 1;
                    cur = (*cur).next;
                }
            }
        }
        for i in 0..other.capacity {
            let mut cur = unsafe { (*other.data.add(i)).head };
            while !cur.is_null() {
                unsafe {
                    result.add(&(*cur).data);
                    cur = (*cur).next;
                }
            }
        }
        result
    }

    pub fn intersect(&self, other: &Set<T>) -> Set<T> {
        let mut result = Set::new();
        for i in 0..self.capacity {
            let mut cur = unsafe { (*self.data.add(i)).head };
            while !cur.is_null() {
                unsafe {
                    if other.contains(&(*cur).data) {
                        result.add(&(*cur).data);
                    }
                    cur = (*cur).next;
                }
            }
        }
        result
    }

    pub fn difference(&self, other: &Set<T>) -> Set<T> {
        let mut result = Set::new();
        for i in 0..self.capacity {
            let mut cur = unsafe { (*self.data.add(i)).head };
            while !cur.is_null() {
                unsafe {
                    if !other.contains(&(*cur).data) {
                        result.add(&(*cur).data);
                    }
                    cur = (*cur).next;
                }
            }
        }
        result
    }

    pub fn print(&self) where T: Display {
        for i in 0..self.capacity {
            unsafe {
                if (*self.data.add(i)).head.is_null() { continue; }
                print!("[{}] ", i);
                (*self.data.add(i)).print();
            }
        }
    }

    pub fn save(&self, os: &mut impl Write) where T: Display {
        for i in 0..self.capacity {
            unsafe { (*self.data.add(i)).save(os); }
        }
    }
}

impl<T> Drop for Set<T> {
    fn drop(&mut self) {
        deallocBuckets(self.data, self.capacity);
    }
}