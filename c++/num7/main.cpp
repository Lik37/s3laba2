#include "../structs/lru_cache.h"
#include <iostream>
#include <string>

using namespace std;


int main() {
    size_t cap, q;
    cin >> cap >> q;

    LRUCache<int, int> cache(cap);

    for (size_t i = 0; i < q; ++i) {
        string op;
        cin >> op;

        if (op == "SET") {
            int x, y;
            cin >> x >> y;
            cache.set(x, y);
        } else if (op == "GET") {
            int x;
            cin >> x;

            int* val = cache.get(x);
            if (val != nullptr) {
                cout << *val << "\n";
            } else {
                cout << "-1\n";
            }
        }
    }

    return 0;
}