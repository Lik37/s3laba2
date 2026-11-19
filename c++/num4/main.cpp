#include "../structs/array_list.h"

#include <iostream>
#include <string>

using namespace std;


bool matchPattern(const ArrayList<char>& s, const ArrayList<char>& r) {
    size_t si = 0, ri = 0;
    bool hasStar = false;
    size_t starPos = 0;
    size_t starMatch = 0;

    while (si < s.getSize()) {
        if ( ri < r.getSize() && (r[ri] == '?' || r[ri] == s[si]) ) {
            si++;
            ri++;
        }
        else if ( ri < r.getSize() && r[ri] == '*' ) {
            hasStar = true;
            starPos = ri; 
            ri++;
            starMatch = si;
        }
        else if ( hasStar ) {
            ri = starPos + 1;
            starMatch++;
            si = starMatch;
        }
        else {
            return false;
        }
    }

    for (;ri < r.getSize() && r[ri] == '*'; ri++);

    return ri == r.getSize();
}


int main(int argc, char* argv[]) {
    if (argc < 3) {
        cerr << "Usage: ./main <string> <pattern>\n";
        return 1;
    }

    string sStr = argv[1];
    string rStr = argv[2];

    ArrayList<char> s;
    ArrayList<char> r;

    for (char c : sStr) 
        s.pushBack(c);
    for (char c : rStr) 
        r.pushBack(c);

    if (matchPattern(s, r))
        cout << "TRUE\n";
    else
        cout << "FALSE\n";

    return 0;
}