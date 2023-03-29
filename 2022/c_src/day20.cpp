#include <stdio.h>
#include <stdlib.h>
#include <vector>

namespace aoc22 {
namespace day20 {

struct LinkedNode {
    int val;
    LinkedNode *prv;
    LinkedNode *nxt;
};

int part1(std::vector<int> &inputs)
{
    std::vector<LinkedNode*> nodes;
    for (int i = 0; i < inputs.size(); i++) {
        nodes.push_back(new LinkedNode { inputs[i], nullptr, nullptr });
    }
    nodes[0]->prv = nodes[nodes.size()-1];
    nodes[0]->nxt = nodes[1];
    nodes[nodes.size()-1]->prv = nodes[nodes.size()-2];
    nodes[nodes.size()-1]->nxt = nodes[0];
    // printf("%d. prev: %d next: %d\n", nodes[0]->val, nodes[0]->prv->val,
        // nodes[0]->nxt->val);
    for (int i = 1; i < inputs.size()-1; i++) {
        nodes[i]->prv = nodes[i-1];
        nodes[i]->nxt = nodes[i+1];
        // printf("%d. prev: %d next: %d\n", nodes[i]->val, nodes[i]->prv->val,
            // nodes[i]->nxt->val);
    }
    // printf("%d. prev: %d next: %d\n", nodes[inputs.size()-1]->val, nodes[inputs.size()-1]->prv->val,
        // nodes[inputs.size()-1]->nxt->val);
    LinkedNode *current, *old_prv, *old_nxt, *new_prv, *new_nxt;
    for (int i = 0; i < nodes.size(); i++) {
        current = nodes[i];
        // printf("moving\n");
        // printf("%d. prev: %d next: %d\n", current->val, current->prv->val,
            // current->nxt->val);
        // printf("to\n");
        int cycle = nodes[i]->val;
        // Connect hole in old position
        current->prv->nxt = current->nxt;
        current->nxt->prv = current->prv;
        new_prv = current->prv;
        for (int ii = 0; ii < abs(cycle); ii++) {
            new_prv = cycle > 0 ? new_prv->nxt : new_prv->prv;
        }
        new_nxt = new_prv->nxt;
        current->prv = new_prv;
        current->nxt = new_nxt;
        new_prv->nxt = current;
        new_nxt->prv = current;
        // printf("%d. prev: %d next: %d\n", current->val, current->prv->val,
            // current->nxt->val);
        LinkedNode *s = nodes[0];
        LinkedNode *t = s->nxt;
        // printf("%d ", s->val);
        while (t != s) {
            // printf("%d ", t->val);
            t = t->nxt;
        }
        // printf("\n");
    }

    // TODO: find starting point in the circular list
    int i1 = 1000 % nodes.size();
    int i2 = 2000 % nodes.size();
    int i3 = 3000 % nodes.size();
    // printf("find 0\n");
    for (int i = 0; i < nodes.size(); i++) {
        // printf("%d. prev: %d next: %d\n", nodes[i]->val, nodes[i]->prv->val,
            // nodes[i]->nxt->val);
        if (nodes[i]->val == 0) {
            current = nodes[i];
            break;
        }
    }
    std::vector<int> outputs{current->val};
    // printf("0 ");
    LinkedNode *z = current;
    current = current->nxt;
    while (current != z) {
        // printf("%d ", current->val);
        outputs.push_back(current->val);
        current = current->nxt;
    }
    // printf("\n");
    return outputs[i1] + outputs[i2] + outputs[i3];
}


} // namespace day20
} // namespace aoc22

int main() {
    {std::vector<int> test_input{1,2,-3, 3,-2,0,4};
    int res = aoc22::day20::part1(test_input);
    printf("part 1 test: %u\n", res);}
    {std::vector<int> test_input{1,2,3, 3,-2,0,4};
    int res = aoc22::day20::part1(test_input);
    printf("part 1 test: %u\n", res);}
    {std::vector<int> test_input{1,10,-8, 3,-2,0,5};
    int res = aoc22::day20::part1(test_input);
    printf("part 1 test: %u\n", res);}
    {std::vector<int> test_input{1,16,-8, 3,-12,0,25};
    int res = aoc22::day20::part1(test_input);
    printf("part 1 test: %u\n", res);}

    FILE *fp = fopen("../inputs/day1.txt", "r");
    size_t len = 0;
    ssize_t r;
    char *line = NULL;
    std::vector<int> inputs;
    while ((r = getline(&line, &len, fp)) != -1)  {
        int a;
        sscanf(line, "%d", &a);
        inputs.push_back(a);
    }

    int res = aoc22::day20::part1(inputs);
    printf("part 1: %u\n", res);
}
