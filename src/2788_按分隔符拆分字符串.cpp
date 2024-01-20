#include <iostream>
#include <sstream>
#include <string>
#include <vector>
using namespace std;

class Solution
{
public:
    vector<string> splitWordsBySeparator(vector<string> &words, char separator)
    {
        vector<string> result;

        for (const auto &word : words)
        {
            stringstream stream(word);
            string substring;

            while (getline(stream, substring, separator))
            {
                if (!substring.empty())
                {
                    result.push_back(substring);
                }
            }
        }

        return result;
    }
};
