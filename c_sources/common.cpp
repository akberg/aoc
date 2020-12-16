
vector<string> general_input(int day)
{
    ifstream ifile;
    vector<string> out;
    string line;
    ifile.open("inputs/day" + to_string(1) + ".txt");
    if (ifile.is_open()) {
        while (getline(ifile, line)) {
            out.push_back(line);
        }
    }
    ifile.close();
    return out;
}