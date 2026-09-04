#include <bits/stdc++.h>
#include <vector>
using namespace std;
#include "../process.h"

void sjf(vector<Process>& P){
    int current_time = 0;   
    int completed = 0;
    int n = P.size();
    vector<bool> done(n,false);
    
    while(completed!=P.size()){
        int idx = -1;
        for(int i=0;i<P.size();i++){
            if(P[i].at <= current_time && !done[i]){
                if(idx==-1 || P[idx].bt > P[i].bt || (P[idx].bt == P[i].bt && P[idx].pid > P[i].pid)){
                    idx=i;
                }
            }
        }

        // cpu idle
        if(idx==-1){
            current_time++;
            continue;
        }
        current_time+=P[idx].bt;
        P[idx].ct = current_time;
        P[idx].tat = P[idx].ct - P[idx].at;
        P[idx].wt = P[idx].tat - P[idx].bt;
        completed++;
        done[idx]= true;
    }
}