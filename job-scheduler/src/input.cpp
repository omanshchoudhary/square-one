#include <bits/stdc++.h>
using namespace std;
#include "process.h"

void fcfs(vector<Process> &P);
void sjf(vector<Process> &P);
void srtf(vector<Process> &P);
void rr(vector<Process> &P, int quantum);
void preemptivePriority(vector<Process> &P);

static void printTable(vector<Process> &P) {
    for (auto &p : P) {
        cout << p.pid << " " << p.at << " " << p.bt << " " << p.ct << " " << p.tat << " " << p.wt << "\n";
    }
}

void useFCFS() {
    int n;
    cin >> n;
    vector<Process> P(n);
    for (int i = 0; i < n; i++) {
        cin >> P[i].pid >> P[i].at >> P[i].bt;
        P[i].ct = P[i].tat = P[i].wt = 0;
    }
    fcfs(P);
    printTable(P);
}

void useSJF() {
    int n;
    cin >> n;
    vector<Process> P(n);
    for (int i = 0; i < n; i++) {
        cin >> P[i].pid >> P[i].at >> P[i].bt;
        P[i].ct = P[i].tat = P[i].wt = 0;
    }
    sjf(P);
    printTable(P);
}

void useSRTF() {
    int n;
    cin >> n;
    vector<Process> P(n);
    for (int i = 0; i < n; i++) {
        cin >> P[i].pid >> P[i].at >> P[i].bt;
        P[i].rem = P[i].bt;
        P[i].ct = P[i].tat = P[i].wt = 0;
    }
    srtf(P);
    printTable(P);
}

void useRR() {
    int n;
    cin >> n;
    vector<Process> P(n);
    for (int i = 0; i < n; i++) {
        cin >> P[i].pid >> P[i].at >> P[i].bt;
        P[i].rem = P[i].bt;
        P[i].ct = P[i].tat = P[i].wt = 0;
    }
    int quantum;
    cin >> quantum;
    rr(P, quantum);
    printTable(P);
}

void usePreemptivePriority() {
    int n;
    cin >> n;
    vector<Process> P(n);
    for (int i = 0; i < n; i++) {
        cin >> P[i].pid >> P[i].at >> P[i].bt >> P[i].priority;
        P[i].rem = P[i].bt;
        P[i].ct = P[i].tat = P[i].wt = 0;
    }
    preemptivePriority(P);
    printTable(P);
}
