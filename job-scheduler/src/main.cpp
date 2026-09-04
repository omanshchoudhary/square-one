#include <bits/stdc++.h>
using namespace std;
#include "input.h"

int main() {
    int choice;
    cin >> choice;
    switch (choice) {
        case 1: useFCFS(); break;
        case 2: useSJF(); break;
        case 3: useSRTF(); break;
        case 4: useRR(); break;
        case 5: usePriority(); break;
    }
    return 0;
}
