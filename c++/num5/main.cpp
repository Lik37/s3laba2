#include <iostream>
#include <string>
#include <vector>

using namespace std;


struct HNode {
    char c;
    int w;
    HNode* left;
    HNode* right;

    HNode(char c, int w)
        : c(c)
        , w(w)
        , left(nullptr)
        , right(nullptr)
    {}
    HNode(int w, HNode* l, HNode* r) 
        : c(0)
        , w(w)
        , left(l)
        , right(r) 
    {}

    bool isLeaf() const { 
        return left == nullptr && right == nullptr; 
    }
};


bool getCode(const HNode* node, char c, const string& path, string& result) {
    if (node == nullptr) return false;

    if (node->isLeaf()) {
        if (node->c == c) {
            result = path.empty() ? "0" : path;
            return true;
        }
        return false;
    }

    if (getCode(node->left, c, path + '0', result))
        return true;

    if (getCode(node->right, c, path + '1', result))
        return true;

    return false;
}


HNode* buildTree(const string& s) {
    int cnts[256] = {0};
    for (unsigned char c : s) 
        cnts[c]++;

    vector<HNode*> nodes;
    for (int i = 0; i < 256; i++) {
        if (cnts[i] > 0) {
            nodes.push_back(new HNode(i, cnts[i]));
        }
    }

    while (nodes.size() > 1) {
        size_t i1 = 0, i2 = 1;
        if (nodes[i1]->w > nodes[i2]->w)
            swap(i1, i2);

        for (size_t i = 2; i < nodes.size(); i++) {
            if (nodes[i]->w < nodes[i1]->w) {
                i2 = i1;
                i1 = i;
            } else if (nodes[i]->w < nodes[i2]->w) {
                i2 = i;
            }
        }

        HNode* a = nodes[i1];
        HNode* b = nodes[i2];

        if (i1 > i2) swap(i1, i2);
        nodes.erase(nodes.begin() + i2);
        nodes.erase(nodes.begin() + i1);

        nodes.push_back(new HNode(a->w + b->w, a, b));
    }

    return nodes[0];
}


string encode(const string& s, HNode* root) {
    string result;
    for (char c : s) {
        string path, code;
        getCode(root, c, path, code);
        result += code;
    }
    return result;
}


string decode(const string& bits, HNode* root) {
    if (root->isLeaf()) {
        return string(bits.size(), root->c);
    }

    string result;
    HNode* cur = root;
    for (char b : bits) {
        cur = (b == '0') ? cur->left : cur->right;
        if (cur->isLeaf()) {
            result += cur->c;
            cur = root;
        }
    }
    return result;
}


int main(int argc, char* argv[]) {
    if (argc < 2) {
        cerr << "usage: ./main <string>\n";
        return 1;
    }

    string s = argv[1];
    if (s.empty()) {
        cout << "empty\n";
        return 0;
    }

    HNode* root = buildTree(s);
    string encoded = encode(s, root);
    string decoded = decode(encoded, root);

    cout << "encoded: " << encoded << "\n";
    cout << "decoded: " << decoded << "\n";
}