#include <stack>

class Node {
public:
    int val;
    Node *next;

    Node() {
        val = 0;
        next = nullptr;
    }

    Node(int val) {
        this->val = val;
        this->next = nullptr;
    }

    Node(int val, Node *next) {
        this->val = val;
        this->next = next;
    }
};

class Solution {
public:
    Node *insert(Node *head, int insertVal) {
        if (head == nullptr) {
            auto *ret = new Node(insertVal);
            ret->next = ret;
            return ret;
        }
        auto *currentNode = head->next;

        while (true) {
            if (insertVal == currentNode->val || currentNode == head) {
                auto *newNode = new Node(insertVal);
                newNode->next = currentNode->next;
                currentNode->next = newNode;
                break;
            } else if (insertVal > currentNode->val) {
                if (insertVal < currentNode->next->val || currentNode->next->val < currentNode->val) {
                    auto *newNode = new Node(insertVal);
                    newNode->next = currentNode->next;
                    currentNode->next = newNode;
                    break;
                }
            } else {
                if (currentNode->next->val < currentNode->val && insertVal < currentNode->next->val) {
                    auto *newNode = new Node(insertVal);
                    newNode->next = currentNode->next;
                    currentNode->next = newNode;
                    break;
                }
            }
            currentNode = currentNode->next;
        }

        return head;
    }
};

int main() {

}