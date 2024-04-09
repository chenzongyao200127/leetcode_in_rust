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
// However, to provide a one-to-one translation using std::deque, here's how you can implement it:‘

// 实现的不优美，而且超时
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
#include <list>

// In this C++ version:
using namespace std;

class LRUCache
{
private:
    int capacity;
    // 注意这个结构的定义
    unordered_map<int, pair<int, std::list<int>::iterator>> cache;
    list<int> keys;

    void touch(int key)
    {
        auto &it = cache[key].second; // 这行代码从缓存中取出与 key 关联的迭代器。迭代器 it 指向了 keys 列表中的元素，这个元素表示了 key 的最近一次访问位置。
        keys.erase(it);               // 从 keys 列表中移除当前迭代器 it 所指向的元素。这个操作是为了从访问顺序中移除 key，因为 key 刚刚被访问了，所以它需要被移动到列表的前端，表示它现在是最近访问的。
        keys.push_front(key);         // 在 keys 列表的前端插入 key。由于我们刚刚访问了 key，它现在应该是最新访问的元素，因此它被放置在列表的最前面。
        it = keys.begin();            // 更新 cache 中的迭代器，使其指向 keys 列表中的新位置（即前端）。这是必要的，因为我们刚刚改变了 key 在 keys 列表中的位置。如果不更新这个迭代器，cache 中存储的迭代器就会失效，以后我们就无法正确地删除或访问这个键了。
    }

public:
    explicit LRUCache(int capacity) : capacity(capacity) {}

    int get(int key)
    {
        // 不符合O(1)的要求
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

#include <unordered_map>
#include <memory>

class LRUCache
{
public:
    struct Node
    {
        int key;
        int val;
        std::shared_ptr<Node> next;
        std::weak_ptr<Node> prev;

        Node(int k, int v) : key(k), val(v), next(nullptr), prev() {}
    };

private:
    int capacity_;
    std::unordered_map<int, std::shared_ptr<Node>> node_map;
    std::shared_ptr<Node> head_;
    std::shared_ptr<Node> tail_;

    void prepend(std::shared_ptr<Node> &n)
    {
        n->next = head_->next;
        n->prev = head_;
        head_->next->prev = n;
        head_->next = n;
    }

    void update(std::shared_ptr<Node> &n)
    {
        auto prev = n->prev.lock();
        auto next = n->next;
        prev->next = next;
        next->prev = prev;
        prepend(n);
    }

public:
    explicit LRUCache(int capacity) : capacity_(capacity)
    {
        if (capacity <= 0)
        {
            throw std::invalid_argument("Capacity must be positive");
        }
        head_ = std::make_shared<Node>(0, 0);
        tail_ = std::make_shared<Node>(0, 0);
        head_->next = tail_;
        tail_->prev = head_;
    }

    int get(int key)
    {
        auto it = node_map.find(key);
        if (it != node_map.end())
        {
            update(it->second);
            return it->second->val;
        }
        return -1;
    }

    void put(int key, int value)
    {
        auto it = node_map.find(key);
        if (it != node_map.end())
        {
            it->second->val = value;
            update(it->second);
        }
        else
        {
            if (node_map.size() == capacity_)
            {
                auto lru = tail_->prev.lock();
                node_map.erase(lru->key);
                lru->val = value;
                lru->key = key;
                update(lru);
                node_map[key] = lru;
            }
            else
            {
                auto n = std::make_shared<Node>(key, value);
                prepend(n);
                node_map[key] = n;
            }
        }
    }

    ~LRUCache()
    {
        // Smart pointers automatically clean up
    }
};

/**
 * Your LRUCache object will be instantiated and called as such:
 * LRUCache* obj = new LRUCache(capacity);
 * int param_1 = obj->get(key);
 * obj->put(key,value);
 */