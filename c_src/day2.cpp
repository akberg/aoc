#include <fstream>
#include <vector>
#include <string>
#include <iostream>
#include <regex>
#include <algorithm>

using namespace std;

typedef struct policy {
    int ll;
    int ul;
    char chr;
    string pwd;
} policy_t;

vector<policy_t> input()
{
    ifstream ifile;
    vector<policy_t> out;
    string line;
    ifile.open("inputs/day2.txt");
    if (ifile.is_open()) {
        regex re("(\\d+)-(\\d+) ([a-z]): ([a-z]+)");
        smatch m;
        while (getline(ifile, line)) {
            if (regex_search(line, m, re)) {
                out.push_back((policy_t){
                    stoi(m.str(1)),
                    stoi(m.str(2)),
                    m.str(3)[0],
                    m.str(4)
                    });
            }
        }
    }
    ifile.close();
    return out;
}

int part1(vector<policy_t> inputs)
{
    int count = 0;
    for (auto p : inputs) {
        int n = 0;
        for (char c : p.pwd)
            n += c==p.chr ? 1 : 0;
        if (p.ll <= n && n <= p.ul)
            count++;
    }
    return count;
}

int part2(vector<policy_t> inputs)
{
    int count = 0;
    for (auto p : inputs) {
        if ((p.pwd[p.ll-1] == p.chr) ^ (p.pwd[p.ul-1] == p.chr))
            count++;
    }
    return count;
}

int main(void)
{
    vector<policy_t>inputs = input();
    policy_t p = inputs.at(0);
    cout << part1(inputs) << endl;
    cout << part2(inputs) << endl;
}