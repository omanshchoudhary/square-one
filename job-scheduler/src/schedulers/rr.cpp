#include<bits/stdc++.h>
#include <queue>
#include <vector>
using namespace std;
#include "../process.h"

void rr(vector<Process>& P, int quantum) {
    int current_time = 0;
    int completed = 0;
    int n = P.size();

    sort(P.begin(), P.end(), [](const Process& a, const Process& b){
        if(a.at != b.at) return a.at<b.at;
        return a.pid < b.pid;
    });
    
    queue<int> q;
    vector<bool> inqueue(n, false);
    while (completed!=n) {
        while (!q.empty()) {
            int idx = q.front();
            q.pop();
            inqueue[idx] = false;
            for(int qt=0;qt<quantum;qt++){
                P[idx].rem--;
                current_time++;

                for(int i=0;i<n;i++){
                    // sorted P keeps queue in correct order
                    if(P[i].at <= current_time && !inqueue[i] && P[i].rem > 0 && i != idx){
                        q.push(i);
                        inqueue[i] = true;
                    }
                }

                if(P[idx].rem==0){
                    P[idx].ct = current_time;
                    P[idx].tat = P[idx].ct - P[idx].at;
                    P[idx].wt = P[idx].tat - P[idx].bt;
                    completed++;
                    break;
                }
            }
            if(P[idx].rem > 0) {
                q.push(idx);
                inqueue[idx] = true;
            }
        }
        for(int i=0; i<n; i++) {
            if(P[i].at >= current_time && P[i].rem > 0 && !inqueue[i]) {
                current_time=P[i].at;
                q.push(i);
                inqueue[i] = true;
                break;
            }
        }
    }
}
