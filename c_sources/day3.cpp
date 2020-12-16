#include <vector>
#include <iterator>
#include <fstream>
#include <iostream>
#include <string>

using namespace std;

vector<vector<bool>> input()
{
    vector<vector<bool>> out;
    ifstream ifile;
    string line;
    ifile.open("inputs/day3.txt");
    if (ifile.is_open()) {
        while (getline(ifile, line)) {
            vector<bool> row;
            for (char c : line) {
                row.push_back(c == '#');
            }
            out.push_back(row);
        }
    }

    return out;
}

int slope(vector<vector<bool>> inputs, int dx, int dy)
{
    unsigned int width = inputs.at(0).size();
    unsigned int cx = 0, cy = 0;
    int count = 0;
    while (cy < inputs.size()) {
        count += inputs.at(cy).at(cx % width) ? 1 : 0;
        cx += dx;
        cy += dy;
    }
    return count;
}

int part1(vector<vector<bool>> inputs)
{
    return slope(inputs, 3, 1);
}

uint64_t part2(vector<vector<bool>> inputs)
{
    return (uint64_t)slope(inputs, 1, 1) *
        (uint64_t)slope(inputs, 3, 1) *
        (uint64_t)slope(inputs, 5, 1) *
        (uint64_t)slope(inputs, 7, 1) *
        (uint64_t)slope(inputs, 1, 2);
}

int main(void)
{
    cout << "Reading inputs" << endl;
    vector<vector<bool>> inputs = input();
    cout << "Day 3 part 1: " << endl;
    cout << part1(inputs) << endl;
    cout << "Day 3 part 2: " << endl;
    cout << part2(inputs) << endl;
}