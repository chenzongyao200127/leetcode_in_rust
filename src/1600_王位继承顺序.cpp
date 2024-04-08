// 1600_王位继承顺序
// https://leetcode.cn/problems/throne-inheritance/description/
#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <string>
#include <functional>

class ThroneInheritance
{
private:
    std::unordered_map<std::string, std::vector<std::string>> edges;
    std::unordered_set<std::string> dead;
    std::string king;

public:
    ThroneInheritance(std::string kingName) : king(std::move(kingName)) {}

    void birth(const std::string &parentName, const std::string &childName)
    {
        edges[parentName].emplace_back(childName);
    }

    void death(const std::string &name)
    {
        dead.insert(name);
    }

    std::vector<std::string> getInheritanceOrder()
    {
        std::vector<std::string> inheritanceOrder;
        std::function<void(const std::string &)> preorder;
        preorder = [this, &inheritanceOrder, &preorder](const std::string &name)
        {
            if (dead.find(name) == dead.end())
            {
                inheritanceOrder.push_back(name);
            }
            for (const auto &childName : edges[name])
            {
                preorder(childName);
            }
        };

        preorder(king);
        return inheritanceOrder;
    }
};