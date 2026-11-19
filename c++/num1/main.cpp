#include "../structs/stack.h"

#include <iostream>
#include <string>
#include <stdexcept>

using namespace std;


void solveTop(Stack<int>& nums, Stack<char>& opers) {
    if (opers.isEmpty() || opers.front() == '(') {
        throw runtime_error("expected operator");
    }

    if (nums.isEmpty()) {
        throw runtime_error("not enough operands");
    }

    int b = nums.pop();

    if (nums.isEmpty()) {
        throw runtime_error("not enough operands");
    }

    int a = nums.pop();

    int res = 0;
    switch (opers.pop()) {
        case '*':
            res = a * b;
            break;
        case '/':
            if (b == 0) {
                throw runtime_error("division by zero");
            }
            res = a / b;
            break;
        case '+':
            res = a + b;
            break;
        case '-':
            res = a - b;
            break;
        default:
            throw runtime_error("unknown operator");
    }
    nums.push(res);
}


bool popNumBuff(string& str, int& out) {
    if (str.empty()) {
        return false;
    }

    if (str == "-") {
        throw runtime_error("dangling unary minus");
    }

    size_t pos = 0;
    try {
        out = stoi(str, &pos);
    }
    catch (const exception&) {
        throw runtime_error("invalid number: '" + str + "'");
    }

    if (pos != str.size()) {
        throw runtime_error("invalid number: '" + str + "'");
    }

    str.clear();
    return true;
}


int main() {
    try {
        string str;
        getline(cin, str);

        Stack<int> nums;
        Stack<char> opers;

        string strNum;
        int curNum;
        bool nextUnary = true;

        for (char ch : str) {
            switch (ch) {
            case ' ':
                break;

            case '*':
            case '/':
                if (popNumBuff(strNum, curNum)) {
                    nums.push(curNum);
                }
                if (!opers.isEmpty() && opers.front() != '(' &&
                    (opers.front() == '*' || opers.front() == '/')) {
                    solveTop(nums, opers);
                }
                opers.push(ch);
                nextUnary = true;
                break;

            case '+':
            case '-':
                if (ch == '-' && nextUnary) {
                    strNum += '-';
                    break;
                }
                if (popNumBuff(strNum, curNum)) {
                    nums.push(curNum);
                }
                if (!opers.isEmpty() && opers.front() != '(') {
                    solveTop(nums, opers);
                }
                opers.push(ch);
                nextUnary = true;
                break;

            case '(':
                if (strNum == "-") {
                    nums.push(-1);
                    opers.push('*');
                    strNum.clear();
                }
                opers.push(ch);
                nextUnary = true;
                break;

            case ')':
                if (popNumBuff(strNum, curNum)) {
                    nums.push(curNum);
                }
                while (!opers.isEmpty() && opers.front() != '(') {
                    solveTop(nums, opers);
                }
                if (opers.isEmpty()) {
                    throw runtime_error("unmatched ')'");
                }
                opers.pop();
                nextUnary = false;
                break;

            default:
                strNum.push_back(ch);
                nextUnary = false;
                break;
            }
        }

        if (popNumBuff(strNum, curNum)) {
            nums.push(curNum);
        }

        while (!opers.isEmpty()) {
            if (opers.front() == '(') {
                throw runtime_error("unmatched '('");
            }
            solveTop(nums, opers);
        }

        if (nums.isEmpty()) {
            throw runtime_error("empty expression");
        }

        int result = nums.pop();

        if (!nums.isEmpty()) {
            throw runtime_error("trailing operands in expression");
        }

        cout << "solve: " << result << "\n";
    }
    catch (const exception& e) {
        cerr << "error: " << e.what() << "\n";
        return 1;
    }

    return 0;
}