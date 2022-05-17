#include <stack>

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;

    explicit TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
};

using namespace std;

class Solution {
public:
    TreeNode *getTargetCopy(TreeNode *original, TreeNode *cloned, TreeNode *target) {
        stack<TreeNode *> oStack{};
        stack<TreeNode *> cStack{};
        stack<int> childStack{};

        TreeNode *currentO = original;
        TreeNode *currentC = cloned;

        childStack.push(-1);
        while (true) {
            if (currentO == target) {
                return currentC;
            }
            int top = childStack.top();
            printf("currentO=%d top=%d l:%p r:%p\n", currentO->val, top, currentO->left, currentO->right);
            if (top == -1) {
                if (currentO->left != nullptr) {
                    oStack.push(currentO);
                    cStack.push(currentC);
                    childStack.push(-1);
                    currentO = currentO->left;
                    currentC = currentC->left;
                } else {
                    childStack.pop();
                    childStack.push(1);
                }
            } else if (top == 1) {
                if (currentO->right != nullptr) {
                    oStack.push(currentO);
                    cStack.push(currentC);
                    childStack.push(-1);
                    currentO = currentO->right;
                    currentC = currentC->right;
                } else {
                    childStack.pop();
                    childStack.push(0);
                }
            } else {
                childStack.pop();
                if (childStack.empty()) {
                    return nullptr;
                }
                currentO = oStack.top();
                oStack.pop();
                currentC = cStack.top();
                cStack.pop();
                top = childStack.top();
                if (top == -1) {
                    childStack.pop();
                    childStack.push(1);
                } else if (top == 1) {
                    childStack.pop();
                    childStack.push(0);
                }
            }
        }
    }
};





TreeNode* copy(TreeNode* root) {
    if (root == nullptr) {
        return nullptr;
    }
    auto* ret = new TreeNode(root->val);
    ret->left = copy(root->left);
    ret->right = copy(root->right);
    return ret;
}



int main() {
    auto* root = new TreeNode(7);
    root->left = new TreeNode(4);
    auto* target = new TreeNode(3);
    target->left = new TreeNode(6);
    target->right = new TreeNode(19);
    root->right = target;
    auto* copyRoot = copy(root);
    printf("target: %p result: %p\n", target, Solution{}.getTargetCopy(root, copyRoot, new TreeNode(0)));
    return 0;
}