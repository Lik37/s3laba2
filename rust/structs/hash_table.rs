use super::array_list::ArrayList;
use super::hash::Hashable;
use std::fmt::Display;
use std::io::Write;

#[derive(Clone, Copy, PartialEq)]
pub enum State { EMPTY, OCCUPIED, DELETED }

pub struct Cell<K, V> {
    pub key: K,
    pub value: V,
    pub state: State,
}

impl<K: Default, V: Default> Default for Cell<K, V> {
    fn default() -> Self {
        Cell { key: K::default(), value: V::default(), state: State::EMPTY }
    }
}

pub fn nextCapacity(n: usize) -> usize { n * 2 - 1 }

pub trait HashTable<K: Hashable, V> {
    fn getProbe(&self, hash: usize, attempt: usize, key: &K) -> usize;

    fn capacity(&self) -> usize;
    fn setCapacity(&mut self, c: usize);
    fn table(&self) -> &ArrayList<Cell<K, V>>;
    fn tableMut(&mut self) -> &mut ArrayList<Cell<K, V>>;

    fn resize(&mut self)
    where
        K: Default + Clone + PartialEq,
        V: Default + Clone,
    {
        let old_cap = self.capacity();
        let old_table = std::mem::replace(self.tableMut(), ArrayList::new());
        self.setCapacity(nextCapacity(old_cap));
        let new_cap = self.capacity();
        for _ in 0..new_cap {
            self.tableMut().pushBack(Cell::default());
        }
        for i in 0..old_cap {
            let state = old_table.get(i).state;
            if state == State::OCCUPIED {
                let k = old_table.get(i).key.clone();
                let v = old_table.get(i).value.clone();
                self.insert(k, v);
            }
        }
    }

    fn insert(&mut self, key: K, value: V)
    where
        K: Default + Clone + PartialEq,
        V: Default + Clone,
    {
        let hash = key.hash(self.capacity());
        for attempt in 0..self.capacity() {
            let i = self.getProbe(hash, attempt, &key);
            let state = self.table().get(i).state;
            if state == State::EMPTY || state == State::DELETED {
                unsafe {
                    let cell = &mut *self.tableMut().data.add(i);
                    cell.key = key;
                    cell.value = value;
                    cell.state = State::OCCUPIED;
                }
                return;
            }
            if state == State::OCCUPIED && self.table().get(i).key == key {
                unsafe {
                    let cell = &mut *self.tableMut().data.add(i);
                    cell.value = value;
                }
                return;
            }
        }
        self.resize();
        self.insert(key, value);
    }

    fn contains(&self, key: &K) -> bool
    where
        K: PartialEq,
    {
        let hash = key.hash(self.capacity());
        for attempt in 0..self.capacity() {
            let idx = self.getProbe(hash, attempt, key);
            let cell = self.table().get(idx);
            if cell.state == State::EMPTY { return false; }
            if cell.state == State::OCCUPIED && &cell.key == key { return true; }
        }
        false
    }

    fn get(&self, key: &K) -> V
    where
        K: PartialEq,
        V: Clone,
    {
        let hash = key.hash(self.capacity());
        for attempt in 0..self.capacity() {
            let idx = self.getProbe(hash, attempt, key);
            let cell = self.table().get(idx);
            if cell.state == State::EMPTY { break; }
            if cell.state == State::OCCUPIED && &cell.key == key {
                return cell.value.clone();
            }
        }
        panic!("key not found");
    }

    fn remove(&mut self, key: &K)
    where
        K: PartialEq,
    {
        let hash = key.hash(self.capacity());
        for attempt in 0..self.capacity() {
            let idx = self.getProbe(hash, attempt, key);
            if self.table().get(idx).state == State::EMPTY { return; }
            if self.table().get(idx).state == State::OCCUPIED
                && &self.table().get(idx).key == key
            {
                unsafe {
                    let cell = &mut *self.tableMut().data.add(idx);
                    cell.state = State::DELETED;
                }
                return;
            }
        }
    }

    fn print(&self)
    where
        K: Display,
        V: Display,
    {
        for i in 0..self.capacity() {
            let cell = self.table().get(i);
            if cell.state == State::OCCUPIED {
                println!("{} {}", cell.key, cell.value);
            }
        }
    }

    fn save(&self, os: &mut impl Write)
    where
        K: Display,
        V: Display,
    {
        for i in 0..self.capacity() {
            let cell = self.table().get(i);
            if cell.state == State::OCCUPIED {
                writeln!(os, "{}", cell.key).unwrap();
                writeln!(os, "{}", cell.value).unwrap();
            }
        }
    }
}

pub struct LinearHashTable<K, V> {
    pub table: ArrayList<Cell<K, V>>,
    pub capacity: usize,
}

impl<K: Default, V: Default> LinearHashTable<K, V> {
    pub fn new() -> Self { Self::withCapacity(3) }

    pub fn withCapacity(cap: usize) -> Self {
        let mut table = ArrayList::new();
        for _ in 0..cap { table.pushBack(Cell::default()); }
        LinearHashTable { table, capacity: cap }
    }
}

impl<K: Hashable, V> HashTable<K, V> for LinearHashTable<K, V> {
    fn getProbe(&self, hash: usize, attempt: usize, _key: &K) -> usize {
        (hash + attempt) % self.capacity
    }
    fn capacity(&self) -> usize { self.capacity }
    fn setCapacity(&mut self, c: usize) { self.capacity = c; }
    fn table(&self) -> &ArrayList<Cell<K, V>> { &self.table }
    fn tableMut(&mut self) -> &mut ArrayList<Cell<K, V>> { &mut self.table }
}

pub struct DoubleHashTable<K, V> {
    pub table: ArrayList<Cell<K, V>>,
    pub capacity: usize,
}

impl<K: Default, V: Default> DoubleHashTable<K, V> {
    pub fn new() -> Self { Self::withCapacity(3) }

    pub fn withCapacity(cap: usize) -> Self {
        let mut table = ArrayList::new();
        for _ in 0..cap { table.pushBack(Cell::default()); }
        DoubleHashTable { table, capacity: cap }
    }
}

impl<K: Hashable, V> HashTable<K, V> for DoubleHashTable<K, V> {
    fn getProbe(&self, hash: usize, attempt: usize, key: &K) -> usize {
        let step = 1 + key.hash(self.capacity - 1);
        (hash + attempt * step) % self.capacity
    }
    fn capacity(&self) -> usize { self.capacity }
    fn setCapacity(&mut self, c: usize) { self.capacity = c; }
    fn table(&self) -> &ArrayList<Cell<K, V>> { &self.table }
    fn tableMut(&mut self) -> &mut ArrayList<Cell<K, V>> { &mut self.table }
}