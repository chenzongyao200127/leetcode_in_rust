# 0806_Hanota_LCCI
from typing import List

class Solution:
    # Method to initiate the Tower of Hanoi process.
    def hanota(self, A: List[int], B: List[int], C: List[int]) -> None:
        n = len(A)  # Get the number of disks in the initial tower (A).
        self.move(n, A, B, C)  # Begin the process of moving disks.

    # Recursive method to move disks between towers.
    def move(self, n, A, B, C):
        if n == 1:
            # Base case: Directly move a single disk from A to C.
            C.append(A[-1])  # Append the top element of A to C.
            A.pop()  # Remove the top element from A.
            return
        else:
            # Recursive case: Move n-1 disks from A to B, using C as auxiliary.
            self.move(n-1, A, C, B)

            # Move the remaining disk from A to C.
            C.append(A[-1])  # Append the top element of A to C.
            A.pop()  # Remove the top element from A.

            # Move the n-1 disks from B to C, using A as auxiliary.
            self.move(n-1, B, A, C)
