//
// Created by plusls on 2022/6/6.
//
#include <stack>

struct ListNode {
    int val;
    ListNode *next;

    ListNode(int x) : val(x), next(nullptr) {}
};

class Solution {
public:
    ListNode *getIntersectionNode(ListNode *headA, ListNode *headB) {
        auto *p1 = headA;
        auto *p2 = headB;
        if (p1 == nullptr || p2 == nullptr) {
            return nullptr;
        }
        while (p1 != p2) {
            p1 = p1->next;
            p2 = p2->next;
            if (p1 == nullptr && p2 == nullptr) {
                break;
            } else if (p1 == nullptr) {
                p1 = headB;
            } else if (p2 == nullptr) {
                p2 = headA;
            }
        }
        return p1;

    }
};

int main() {

}