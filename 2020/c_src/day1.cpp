#include <fstream>
#include <vector>
#include <string>
#include <iostream>

using namespace std;

vector<int> input()
{
    ifstream ifile;
    vector<int> out;
    string line;
    ifile.open("inputs/day" + to_string(1) + ".txt");
    if (ifile.is_open()) {
        while (getline(ifile, line)) {
            out.push_back(stoi(line));
        }
    }
    ifile.close();
    return out;
}

int part1(vector<int> inputs)
{
    for (int i = 0; i < inputs.size(); i++)
        for (int j = i+1; j < inputs.size(); j++)
            if (inputs.at(i) + inputs.at(j) == 2020)
                return inputs.at(i) * inputs.at(j);
    return -1;
}

int part2(vector<int> inputs)
{
    for (int i = 0; i < inputs.size(); i++)
        for (int j = i+1; j < inputs.size(); j++)
            for (int k = j+1; k < inputs.size(); k++)
                if (inputs.at(i) + inputs.at(j) + inputs.at(k) == 2020)
                    return inputs.at(i) * inputs.at(j) * inputs.at(k);
    return -1;
}

int main(void)
{
    vector<int> lines = input();
    int res = part1(lines);
    cout << res << endl;
    res = part2(lines);
    cout << res << endl;
}
