// Множество
// добавление элемента, 
// удаление элемента, 
// проверка наличия элемента.

#pragma once

#include "forward_list.h"
#include "hash.h"

#include <cstddef>
#include <string>
#include <iostream>

template <typename T>
struct Set {
    size_t capacity;
    ForwardList<T>* data;
    size_t size;

    Set() : capacity(8), data(new ForwardList<T>[capacity]), size(0) {}

    Set(const Set& other) : capacity(other.capacity), size(other.size) {
        data = new ForwardList<T>[capacity];
        for (size_t i = 0; i < capacity; i++) {
            ForwardNode<T>* cur = other.data[i].head;
            while (cur != nullptr) {
                data[i].pushBack(cur -> data);
                cur = cur -> next;
            }
        }
    }

    Set& operator=(const Set& other) {
        if (this == &other) return *this;

        delete[] data;
        capacity = other.capacity;
        size = other.size;
        data = new ForwardList<T>[capacity];

        for (size_t i = 0; i < capacity; i++) {
            ForwardNode<T>* cur = other.data[i].head;
            while (cur != nullptr) {
                data[i].pushBack(cur -> data);
                cur = cur -> next;
            }
        }
        return *this;
    }

    ~Set() {
        delete[] data;
    }

    void resize() {
        size_t old_cap = capacity;
        ForwardList<T>* old = data;

        capacity *= 2;
        data = new ForwardList<T>[capacity];
        size = 0;

        for (size_t i = 0; i < old_cap; i++) {
            ForwardNode<T>* cur = old[i].head;
            while (cur != nullptr) {
                add(cur -> data);
                cur = cur -> next;
            }
        }

        delete[] old;
    }

    bool add(const T& value) {
        if (contains(value)) return false;

        if (size + 1 > capacity) {
            resize();
        }

        data[Hash(value, capacity)].pushBack(value);
        size++;
        return true;
    }

    bool contains(const T& value) const {
        return data[Hash(value, capacity)].find(value) != nullptr;
    }

    bool remove(const T& value) {
        if (data[Hash(value, capacity)].deleteValue(value)) {
            size--;
            return true;
        }
        return false;
    }

    bool isEmpty() const {
        return size == 0;
    }

    // задание 3
    Set unite(const Set& other) const { // объединение
        Set result(*this);
        for (size_t i = 0; i < other.capacity; i++) {
            ForwardNode<T>* cur = other.data[i].head;
            while (cur != nullptr) {
                result.add(cur -> data);
                cur = cur -> next;
            }
        }
        return result;
    }

    Set intersect(const Set& other) const { // пересечение
        Set result;
        for (size_t i = 0; i < capacity; i++) {
            ForwardNode<T>* cur = data[i].head;
            while (cur != nullptr) {
                if (other.contains(cur -> data))
                    result.add(cur -> data);
                cur = cur -> next;
            }
        }
        return result;
    }

    Set difference(const Set& other) const { // разность
        Set result;
        for (size_t i = 0; i < capacity; i++) {
            ForwardNode<T>* cur = data[i].head;
            while (cur != nullptr) {
                if (!other.contains(cur -> data))
                    result.add(cur -> data);
                cur = cur -> next;
            }
        }
        return result;
    }

    void print() {
        for (size_t i = 0; i < capacity; i++) {
            if (data[i].head == nullptr) continue;
            std::cout << "[" << i << "] ";
            data[i].print();
        }
    }

    void save(std::ostream& os) const {
        for (size_t i = 0; i < capacity; i++) {
            data[i].save(os);
        }
    }
};