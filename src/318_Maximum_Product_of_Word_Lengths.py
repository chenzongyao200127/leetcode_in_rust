# 318_Maximum_Product_of_Word_Lengths
# https://leetcode.cn/problems/maximum-product-of-word-lengths/description/

# The provided Python code is an optimized solution for finding the maximum product of the lengths of two words such that these words do not share any common letters. Let's break down and explain each part of the code:

# 1. **Import Statements**: 
#    - `from typing import List`: This is for type annotations, enhancing code readability and helping with type checking.
#    - The `defaultdict` and `reduce` functions should also be imported for this code to work.

# 2. **Class and Method Definition**: 
#    - `class Solution`: Defines a solution class.
#    - `def maxProduct(self, words: List[str]) -> int`: Defines a method `maxProduct` that takes a list of words and returns an integer.

# 3. **Creating Bit Masks**:
#    - `masks = defaultdict(int)`: Initializes a default dictionary to store the bit masks.
#    - For each `word` in `words`, a bit mask is created using `reduce` and bitwise operations.
#    - `mask = reduce(lambda a, b: a | (1 << (ord(b) - ord('a'))), word, 0)`: This line creates a bit mask where each bit represents whether a particular letter ('a' to 'z') is present in the word.

# 4. **Storing Maximum Lengths**:
#    - `masks[mask] = max(masks[mask], len(word))`: For each unique mask, the maximum length of a word that corresponds to this mask is stored.

# 5. **Calculating Maximum Product**:
#    - The final return statement calculates the maximum product of lengths of two words that do not share common letters.
#    - It iterates over all pairs of masks using `product(masks, repeat=2)` and checks if the masks have no common bits set (`x & y == 0`). If they don't, it multiplies their corresponding maximum lengths.
#    - `max(..., default=0)` ensures that if no such pair exists, 0 is returned.

# 6. **Missing Imports**:
#    - To make this code functional, you need to add `from collections import defaultdict` and `from functools import reduce` at the beginning of your script. Also, `from itertools import product` is required for the Cartesian product.

# Here's the corrected and complete version of the code:


from typing import List
from collections import defaultdict
from functools import reduce
from itertools import product

class Solution:
    def maxProduct(self, words: List[str]) -> int:
        masks = defaultdict(int)
        for word in words:
            mask = reduce(lambda a, b: a | (1 << (ord(b) - ord('a'))), word, 0)
            masks[mask] = max(masks[mask], len(word))
        return max((masks[x] * masks[y] for x, y in product(masks, repeat=2) if x & y == 0), default=0)


# This optimized approach significantly reduces the time complexity compared to a brute-force 
# solution by using bitwise operations and avoiding checking each pair of words directly for common letters.