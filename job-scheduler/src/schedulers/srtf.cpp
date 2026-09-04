#include <bits/stdc++.h>
#include <vector>
using namespace std;
#include "../process.h"

void srtf(vector<Process>& P){
    int current_time = 0 ;
    int completed = 0;
    int n = P.size();
    vector<bool> done(n,false);

    while (completed != n) {
        int idx = -1;
        for(int i=0;i<P.size();i++){
            if(P[i].at <= current_time && !done[i]){
                if(idx==-1 || P[idx].rem > P[i].rem || (P[idx].rem == P[i].rem && P[idx].pid > P[i].pid)){
                    idx=i;
                }
            }
        }
        // cpu idle
        if(idx==-1){
            current_time++;
            continue;
        }

        current_time++;
        P[idx].rem--;

        if(P[idx].rem == 0){
            P[idx].ct = current_time;
            P[idx].tat = P[idx].ct - P[idx].at;
            P[idx].wt = P[idx].tat - P[idx].bt;
            completed++;
            done[idx]= true;
        }
    }
}
