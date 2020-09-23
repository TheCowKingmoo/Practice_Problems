/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

/*
  Main Thoughts:
    We do not know the length of the single linked list and we start at the head.
      -> have to iterate through entire list to get where size - n is
    We have no previous element in the ListNode
      -> need to track last used node to stitch together n-1 and n+1 nodes
    Attempt to do this in one pass
      -> cant just run through the list once to get size and work from there
      -> keep track of current node - n node so that when we reach end of list we already have pointer to the node we want 
 */


class Solution {
public:
    ListNode* removeNthFromEnd(ListNode* head, int n) {
        ListNode *front_runner = head;
        ListNode *tail_runner = head;
        ListNode *tail_runner_previous = NULL;
        int i = 0;
        
       // get the front node that we are check n spaces ahead of tail_runner
        for(i = 0; i < n; i++)  {
             // note n is always valid so no need to check
            front_runner = front_runner->next;
        }
        
        // edge case where n is equal to the length of the list
        if(front_runner == NULL)  {
            return head->next;
        }
        
        // iterate tail_runner by using front runner to know when we hit the end
        while(front_runner != NULL)  {
            front_runner = front_runner->next;
            tail_runner_previous = tail_runner;
            tail_runner = tail_runner->next;
        }
        // remove node a nth place
        tail_runner_previous->next = tail_runner->next;
        return head;
    }
};