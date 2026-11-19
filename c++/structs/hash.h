#pragma once

#include <algorithm>
#include <cstddef>
#include <string>
#include <cstdint>

inline size_t HashString(const std::string& s, size_t size) {
    size_t h = 0;
    for (unsigned char c : s) {
        h = h * 31 + c;
    }
    return h % size;
}

inline size_t HashNumber(int64_t m, size_t size) {
    // constexpr double A = 0.6180339887;
    if (m < 0) m = -m;
    // return static_cast<size_t>(size * ((m * A) - static_cast<size_t>(m * A)));
    return m % size;
}

template <typename T>
inline size_t Hash(const T& key, size_t cap) {
    if constexpr (std::is_same_v<T, std::string>) 
        return HashString(key, cap);
    else 
        return HashNumber(key, cap);
}
