// 2079_给植物浇水
// https://leetcode.cn/problems/watering-plants/description/

#include <vector>

using namespace std;

class Solution
{
public:
    int wateringPlants(vector<int> &plants, int capacity)
    {
        int n = plants.size();
        int refills = 0;
        int water = 0;
        for (int i = 0; i < n; ++i)
        {
            if (water < plants[i])
            {
                water = capacity;
                refills += i * 2;
            }
            water -= plants[i];
            refills++;
        }
        return refills;
    }
};