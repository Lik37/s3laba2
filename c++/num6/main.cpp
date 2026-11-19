#include "../structs/hash_table.h"
#include <iostream>
#include <string>

using namespace std;


int main(int argc, char* argv[]) 
{
    if (argc < 2) {
        cerr << "usage: ./main <number>\n";
        return 1;
    }

    int number;
    try {
        number = stoi(argv[1]);
    }
    catch (...) {
        cerr << "argument is not number";
        return 1;
    }
    
    DoubleHashTable<string, int> dict;
    dict.insert("M", 1000);
    dict.insert("CM", 900);
    dict.insert("D", 500);
    dict.insert("CD", 400);
    dict.insert("C", 100);
    dict.insert("XC", 90);
    dict.insert("L", 50);
    dict.insert("XL", 40);
    dict.insert("X", 10);
    dict.insert("IX", 9);
    dict.insert("V", 5);
    dict.insert("IV", 4);
    dict.insert("I", 1);

    ArrayList<string> symbols;
    symbols.pushBack("M");
    symbols.pushBack("CM");
    symbols.pushBack("D");
    symbols.pushBack("CD");
    symbols.pushBack("C");
    symbols.pushBack("XC");
    symbols.pushBack("L");
    symbols.pushBack("XL");
    symbols.pushBack("X");
    symbols.pushBack("IX");
    symbols.pushBack("V");
    symbols.pushBack("IV");
    symbols.pushBack("I");

    string result = "";

    for (size_t i = 0; i < symbols.size; i++) {
        int val = dict.get(symbols[i]); 
        
        while (number >= val) {
            result += symbols[i];
            number -= val;
        }
    }

    cout << result << "\n";
}