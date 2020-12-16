#include <vector>
#include <iterator>
#include <fstream>
#include <iostream>
#include <string>
#include <regex>

using namespace std;


vector<string> parse_input() 
{
    vector<string> out;
    ifstream ifile("inputs/day4.txt");
    if (ifile.is_open()) {
        string line;
        string psprt = "";
        while (getline(ifile, line)) {
            cout << line.length() << endl;
            if (line.length() > 0) {
                psprt += line;
            } else {
                //cout << psprt << endl;
                out.push_back(psprt);
                psprt = "";
            }
        }
        out.push_back(psprt);
    }
    cout << out.size() << endl;
    return out;
}

void psprt_validate(string inputs, int* sum1, int* sum2) 
{
    cout << inputs << endl;
    string keys[] = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
    int valid = 0;
    int present = 0;
    for (auto k : keys) {
        size_t i = inputs.find(k);
        if (i == std::string::npos) {
            continue;
        } else {
            cout << k << " ";
        }
        present++;
    }
    cout << present << endl;
    *sum1 += present==7 ? 1 : 0;
}

void psprts_validate(vector<string> inputs, int* sum1, int* sum2) 
{
    string keys[] = {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};
    for_each(inputs.begin(), inputs.end(), [&](string const s) {
        int valid = 0;
        int present = 0;
        for (auto k : keys) {
            size_t i = s.find(k);
            if (i == std::string::npos) {
                continue;
            } else {
                cout << k << " ";
            }
            present++;
        }
        cout << present << endl;
        *sum1 += present==7 ? 1 : 0;
    });
}

int main(void)
{
    vector<string> inputs = parse_input();
    int sum1 = 0, sum2 = 0;
    cout << "validating passports" << endl;
    //psprts_validate(inputs, &sum1, &sum2);
    // for (auto psprt : inputs)
    //     psprt_validate(psprt, &sum1, &sum2);
    cout << sum1 << " " << sum2 << endl;
    
}