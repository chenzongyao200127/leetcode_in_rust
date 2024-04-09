// 146_LRU_缓存
// https://leetcode.cn/problems/lru-cache/description/?envType=study-plan-v2&envId=top-100-liked

// Below is the equivalent implementation of the given Rust LRUCache in C++.
// This implementation uses `std::unordered_map` to store key - value pairs
// and `std::list` to maintain the keys' order according to their usage,
// which acts as a queue to determine the least recently used key when eviction is needed.
#include <unordered_map>
#include <deque>
#include <list>

// If you specifically want to use a std::deque (double-ended queue) in C++ to maintain the order of keys,
// keep in mind that this structure does not support efficient random access deletion
// which is required to move an item to the end or front upon access.
// Due to this limitation, the time complexity for the mark_as_recently_used operation would be O(n),
// making the overall LRU cache less efficient.
// However, to provide a one-to-one translation using std::deque, here's how you can implement it:
class LRUCache
{
private:
    int capacity;
    std::unordered_map<int, int> map;
    std::deque<int> order;

    void mark_as_recently_used(int key)
    {
        // Remove key if it exists in the deque
        for (auto it = order.begin(); it != order.end(); ++it)
        {
            if (*it == key)
            {
                order.erase(it);
                break;
            }
        }
        // Add the key to the back to mark it as recently used
        order.push_back(key);
    }

public:
    LRUCache(int capacity) : capacity(capacity) {}

    int get(int key)
    {
        if (map.find(key) != map.end())
        {
            mark_as_recently_used(key);
            return map[key];
        }
        else
        {
            return -1;
        }
    }

    void put(int key, int value)
    {
        if (map.find(key) != map.end())
        {
            // Update the value
            map[key] = value;
            mark_as_recently_used(key);
        }
        else
        {
            if (map.size() == capacity)
            {
                // Evict the least recently used key
                int lru = order.front();
                map.erase(lru);
                order.pop_front();
            }
            map[key] = value;
            order.push_back(key);
        }
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */

#include <unordered_map>
#include <deque>
#include <list>

// In this C++ version:
using namespace std;

class LRUCache
{
private:
    int capacity;
    unordered_map<int, pair<int, std::list<int>::iterator>> cache;
    list<int> keys;

    void touch(int key)
    {
        auto &it = cache[key].second;
        keys.erase(it);
        keys.push_front(key);
        it = keys.begin();
    }

public:
    explicit LRUCache(int capacity) : capacity(capacity) {}

    int get(int key)
    {
        auto cacheIt = cache.find(key);
        if (cacheIt == cache.end())
        {
            return -1;
        }
        else
        {
            touch(key);
            return cacheIt->second.first;
        }
    }

    void put(int key, int value)
    {
        if (cache.find(key) != cache.end())
        {
            cache[key].first = value;
            touch(key);
        }
        else
        {
            if (cache.size() == capacity)
            {
                int lru = keys.back();
                cache.erase(lru);
                keys.pop_back();
            }
            keys.push_front(key);
            cache[key] = {value, keys.begin()};
        }
    }
};

// In this refactored version:

// capacity is explained as the maximum capacity of the cache.
// cache is a map that now has a more descriptive name and a comment explaining its use.
// keys is a list that maintains the order of keys based on their usage.
// The touch method is a new private method to encapsulate the logic of marking a key as recently used.
// This avoids duplication of code in get and put methods and makes them easier to read.
// Descriptive variable names and explicit variable types (auto replaced with explicit type where it may improve clarity).
// The explicit keyword is added to the constructor to prevent implicit conversions.
// Additional comments to explain the logic in the get and put methods.