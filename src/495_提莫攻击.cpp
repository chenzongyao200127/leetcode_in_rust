#include <vector>

class Solution
{
public:
    int findPoisonedDuration(std::vector<int> &timeSeries, int duration)
    {
        if (timeSeries.empty())
            return 0;

        int totalDuration = 0;
        for (int i = 0; i < timeSeries.size() - 1; ++i)
        {
            // Add the minimum of the duration or the interval until the next attack
            totalDuration += std::min(timeSeries[i + 1] - timeSeries[i], duration);
        }

        // Add duration for the last attack
        totalDuration += duration;

        return totalDuration;
    }
};
