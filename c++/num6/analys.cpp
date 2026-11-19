#include "../structs/hash_table.h"

#include <chrono>
#include <iostream>
#include <vector>
#include <random>


std::chrono::steady_clock::time_point start() {
    return std::chrono::steady_clock::now();
}

double stop(std::chrono::steady_clock::time_point t0) {
    auto dt = std::chrono::steady_clock::now() - t0;
    return std::chrono::duration<double, std::micro>(dt).count();
}

template <class Table>
void bench(const char* name, Table& t, const std::vector<uint64_t>& keys, size_t startFill)
{
    const size_t N = 1000;

    // начальное заполнение
    for (size_t i = 0; i < startFill; i++) {
        t.insert(keys[i], i);
    }

    std::cout << "\n\t" << name
              << "\nstart:"
              << "\n  size: " << startFill
              << "\n  capacity: " << t.capacity
              << "\n  alpha: " << double(startFill) / t.capacity;

    size_t pos = startFill;
    auto t0 = start();
    for (size_t i = 0; i < N; i++) {
        t.insert(keys[pos], pos);
        pos++;
    }
    std::cout << "\ninsert:\t\t" << stop(t0) / N;

    t0 = start();
    for (size_t i = 0; i < N; i++) {
        t.contains(keys[i]);
    }
    std::cout << "\nfind good:\t" << stop(t0) / N;

    t0 = start();
    for (size_t i = 0; i < N; i++) {
        t.remove(keys[i]);
    }
    std::cout << "\nremove:\t\t" << stop(t0) / N;

    t0 = start();
    for (size_t i = 0; i < N; i++) {
        t.contains(keys[i]);
    }
    std::cout << "\nfind bad:\t" << stop(t0) / N;

    std::cout << "\nnew capacity:\t" << t.capacity << "\n";
}

int main() {
    double a = 0.8;
    size_t capasity = 100003;
    size_t startFill = size_t(capasity * a);

    std::vector<uint64_t> keys(200000);

    std::mt19937_64 gen;
    for (size_t i = 0; i < keys.size(); i++) {
        keys[i] = gen();
    }

    LinearHashTable<uint64_t, size_t> linearTable(capasity);
    DoubleHashTable<uint64_t, size_t> doubleTable(capasity);

    bench("Linear", linearTable, keys, startFill);
    bench("Double", doubleTable, keys, startFill);

    return 0;
}