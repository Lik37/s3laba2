#pragma once

#include "array_list.h"
#include "hash.h"

#include <iostream>
#include <stdexcept>
#include <type_traits>


enum class State { EMPTY, OCCUPIED, DELETED };

template <typename K, typename V>
struct Cell {
    K key;
    V value;
    State state = State::EMPTY;
};


template <typename K, typename V>
struct HashTable {
    ArrayList<Cell<K, V>> table;
    size_t capacity;

    virtual size_t getProbe(size_t hash, size_t attempt, const K& key) const = 0;

    static size_t nextCapacity(size_t n) {
        return n * 2 - 1;
    }

    void resize() {
        size_t old_cap = capacity;
        capacity = nextCapacity(capacity);
        
        ArrayList<Cell<K, V>> old_table = table;
        table = ArrayList<Cell<K, V>>();
        for (size_t i = 0; i < capacity; i++)
            table.pushBack(Cell<K, V>());
        
        for (size_t i = 0; i < old_cap; i++) {
            if (old_table[i].state == State::OCCUPIED) {
                insert(old_table[i].key, old_table[i].value);
            }
        }
    }

    HashTable() 
            : capacity(3)
    {
        for (size_t i = 0; i < capacity; i++)
            table.pushBack(Cell<K, V>());
    }

    HashTable(size_t init_cap) 
            : capacity(init_cap)
    {
        for (size_t i = 0; i < capacity; i++)
            table.pushBack(Cell<K, V>());
    }
    
    virtual ~HashTable() = default;

    void insert(const K& key, const V& value) {
        size_t hash = Hash(key, capacity);

        for (size_t attempt = 0; attempt < capacity; attempt++) {
            size_t i = getProbe(hash, attempt, key);
            
            if (table[i].state == State::EMPTY || table[i].state == State::DELETED) {
                table[i].key = key;
                table[i].value = value;
                table[i].state = State::OCCUPIED;
                return;
            }
            if (table[i].state == State::OCCUPIED && table[i].key == key) {
                table[i].value = value;
                return;
            }
        }

        resize();
        insert(key, value);
    }

    bool contains(const K& key) const {
        size_t hash = Hash(key, capacity);
        for (size_t attempt = 0; attempt < capacity; attempt++) {
            size_t idx = getProbe(hash, attempt, key);
            if (table[idx].state == State::EMPTY)
                return false;
            if (table[idx].state == State::OCCUPIED && table[idx].key == key)
                return true;
        }
        return false;
    }

    V get(const K& key) const {
        size_t hash = Hash(key, capacity);
        for (size_t attempt = 0; attempt < capacity; attempt++) {
            size_t idx = getProbe(hash, attempt, key);
            if (table[idx].state == State::EMPTY)
                break;
            if (table[idx].state == State::OCCUPIED && table[idx].key == key)
                return table[idx].value;
        }
        throw std::out_of_range("key not found");
    }

    void remove(const K& key) {
        size_t hash = Hash(key, capacity);
        for (size_t attempt = 0; attempt < capacity; attempt++) {
            size_t idx = getProbe(hash, attempt, key);
            if (table[idx].state == State::EMPTY) 
                return;
            if (table[idx].state == State::OCCUPIED && table[idx].key == key) {
                table[idx].state = State::DELETED;
                return;
            }
        }
    }

    void print() const {
        for (size_t i = 0; i < capacity; i++) {
            if (table[i].state == State::OCCUPIED) {
                std::cout << table[i].key << " " << table[i].value << "\n";
            }
        }
    }

    void save(std::ostream& os) const {
        for (size_t i = 0; i < capacity; i++) {
            if (table[i].state == State::OCCUPIED) {
                os << table[i].key << "\n" << table[i].value << "\n";
            }
        }
    }
};


template <typename K, typename V>
struct LinearHashTable : public HashTable<K, V> {
    size_t getProbe(size_t hash, size_t attempt, const K&) const override {
        return (hash + attempt) % this->capacity;
    }

    LinearHashTable() : HashTable<K, V>() {}
    LinearHashTable(size_t cap) : HashTable<K, V>(cap) {}
};

template <typename K, typename V>
struct DoubleHashTable : public HashTable<K, V> {
    size_t getProbe(size_t hash, size_t attempt, const K& key) const override {
        size_t step = 1 + Hash(key, this->capacity - 1);
        return (hash + attempt * step) % this->capacity;
    }

    DoubleHashTable() : HashTable<K, V>() {}
    DoubleHashTable(size_t cap) : HashTable<K, V>(cap) {}
};