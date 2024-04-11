from typing import List


class UnionFind:
    def __init__(self, n: int):
        self.parent = list(range(n))
        self.size = [1] * n
        self.n = n
        self.setCount = n

    def findset(self, x: int) -> int:
        if self.parent[x] != x:
            return self.findset(self.parent[x])
        else:
            return x

    def unite(self, x: int, y: int) -> bool:
        root1 = self.findset(x)
        root2 = self.findset(y)

        # union
        if root1 != root2:
            self.parent[root2] = root1
            self.size[root1] += self.size[root2]
            self.size[root2] = 0
            self.setCount -= 1
            return True
        else:
            return False

    def is_connected(self, x: int, y: int) -> bool:
        return self.findset(x) == self.findset(y)


class Solution:
    def findCircleNum(self, isConnected: List[List[int]]) -> int:
        n = len(isConnected)
        unionFind = UnionFind(n)
        for i in range(n):
            for j in range(n):
                if isConnected[i][j] == 1:
                    unionFind.unite(i, j)

        return unionFind.setCount


# Test Case 1: Initialization
uf = UnionFind(10)
assert uf.setCount == 10, "There should be 10 distinct sets initially."
for i in range(10):
    assert uf.findset(
        i) == i, f"The representative of set {i} should be itself."

# Test Case 2: Union of two elements
uf.unite(0, 1)
assert uf.is_connected(0, 1), "Elements 0 and 1 should be connected."
assert uf.setCount == 9, "There should be 9 distinct sets after uniting 0 and 1."

# Test Case 3: Union of elements that are already connected
result = uf.unite(0, 1)
assert not result, "Union operation should return False if elements are already connected."
assert uf.setCount == 9, "There should still be 9 distinct sets."

# Test Case 4: Union of multiple elements
uf.unite(2, 3)
uf.unite(4, 5)
uf.unite(3, 4)
assert uf.is_connected(
    2, 5), "Elements 2 and 5 should be connected through 3 and 4."
assert uf.setCount == 6, "There should be 6 distinct sets after some unions."

# Test Case 5: Union of elements with different tree sizes
uf.unite(1, 5)
assert uf.size[uf.findset(
    1)] == 6, "The size of the set containing 1 should be 6."
assert uf.is_connected(0, 5), "Elements 0 and 5 should be connected now."
assert uf.setCount == 5, "There should be 5 distinct sets after uniting 1 and 5."

# Test Case 6: Check for no false connections
assert not uf.is_connected(0, 6), "Elements 0 and 6 should not be connected."
assert not uf.is_connected(
    7, 8), "Elements 7 and 8 should not be connected initially."

# Test Case 7: Connect all elements
for i in range(1, 10):
    uf.unite(0, i)
assert uf.setCount == 1, "There should be only 1 set after all unions."
for i in range(1, 10):
    assert uf.is_connected(0, i), f"Element 0 and {i} should be connected."

print("All test cases passed!")
