# 19. Remove Nth Node From End of List

# Definition for singly-linked list.
"""
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
"""
class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        if not head or (not head.next and n == 1):
            return None

        current = head
        count = 1
        while current.next:
            count += 1
            current = current.next

        if n == count:
            return head.next

        current = head
        for _ in range(count - n - 1):
            current = current.next
        current.next = current.next.next
        return head
