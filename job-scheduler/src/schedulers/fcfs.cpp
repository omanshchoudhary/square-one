#include <bits/stdc++.h>
using namespace std;
#include "../process.h"

void fcfs(vector<Process>& P) {
    sort(P.begin(), P.end(), [](const Process& a, const Process& b){
        if(a.at != b.at) return a.at<b.at;
        return a.pid < b.pid;
    });

    int current_time = 0;
    for(int i=0;i<P.size();i++){
        if(current_time<P[i].at){
            current_time=P[i].at;
        }
        current_time+=P[i].bt;
        P[i].ct = current_time;
        P[i].tat = P[i].ct - P[i].at;
        P[i].wt = P[i].tat - P[i].bt;
    }
}