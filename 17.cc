#include <stdio.h>

int main()
{
    int steps;
    scanf("%d", &steps);
    
    int next[2018] = {0};
    int p = 0;
    for (int i = 1; i <= 2017; ++i) {
        for (int j = 0; j < steps; ++j)
            p = next[p];
        next[i] = next[p];
        next[p] = i;
        p = i;
    }
    printf(" *: %d\n", next[p]);
    
    p = 0;
    int after_0 = 0;
    for (int i = 1; i <= 50000000; ++i) {
        p = (p + steps) % i;
        if (p == 0)
            after_0 = i;
        ++p;
    }
    printf("**: %d\n", after_0);
}