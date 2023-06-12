# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

def removeZeroSumSublists(head: Optional[ListNode]) -> Optional[ListNode]:
    nums = []
    while head != None:
        nums.append(head.val)
        head = head.next
    print(nums)
    
removeZeroSumSublists(ListNode())