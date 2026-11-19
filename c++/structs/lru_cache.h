#pragma once

#include "doubly_list.h"
#include "hash_table.h"
#include <stdexcept>

template <typename K, typename V>
struct KVPair {
    K key;
    V value;
};

template <typename K, typename V>
struct LRUCache {
    using Node = DoublyNode<KVPair<K, V>>;

    size_t capacity;
    size_t size = 0;
    DoublyList<KVPair<K, V>> list;
    DoubleHashTable<K, Node*> map;

    LRUCache(size_t cap) : capacity(cap), map(cap) {}

    void unlink(Node* n) {
        if (n->prev != nullptr) {
            n->prev->next = n->next;
        } else {
            list.head = n->next;
        }

        if (n->next != nullptr) {
            n->next->prev = n->prev;
        } else {
            list.tail = n->prev;
        }
    }

    void pushFront(Node* n) {
        n->prev = nullptr;
        n->next = list.head;

        if (list.head != nullptr) {
            list.head->prev = n;
        } else {
            list.tail = n;
        }

        list.head = n;
    }

    void set(const K& key, const V& value) {
        if (capacity == 0) {
            return;
        }

        if (map.contains(key)) 
        {
            Node* n = map.get(key);
            n->data.value = value;
            unlink(n);
            pushFront(n);
        }
        else 
        {   
            if (size == capacity) {
                Node* old = list.tail;
                map.remove(old->data.key);
                unlink(old);
                delete old;
            }
            else 
                size++;
            
            Node* n = new Node({key, value}, nullptr, nullptr);
            pushFront(n);
            map.insert(key, n);
        }
    }

    V* get(const K& key) {
        if (!map.contains(key)) {
            return nullptr;
        }

        Node* n = map.get(key);
        unlink(n);
        pushFront(n);
        return &(n->data.value);
    }
};