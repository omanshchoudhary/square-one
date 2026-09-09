#include <bits/stdc++.h>
using namespace std;
#include "ring.h"

static vector<string> makeKeys(int n) {
    vector<string> keys;
    keys.reserve(static_cast<size_t>(n));
    for (int i = 0; i < n; i++) keys.push_back("key" + to_string(i));
    return keys;
}

static void removalTest() {
    Ring r;
    r.addNode("A", 150);
    r.addNode("B", 150);
    r.addNode("C", 150);

    vector<string> keys = makeKeys(100000);
    vector<string> before;
    before.reserve(keys.size());
    for (const string& k : keys) before.push_back(r.getNode(k));

    r.removeNode("B");

    long fromB = 0;
    long fromOthers = 0;
    for (size_t i = 0; i < keys.size(); i++) {
        if (r.getNode(keys[i]) == before[i]) continue;
        if (before[i] == "B") fromB++;
        else fromOthers++;
    }

    printf("removing node B from a 3 node ring\n");
    printf("  keys that moved off B       %6ld\n", fromB);
    printf("  keys that moved off A or C  %6ld  (want 0)\n", fromOthers);
    printf("  %s\n\n", fromOthers == 0 ? "OK" : "FAIL");
}

static void distributionReport() {
    vector<string> keys = makeKeys(100000);
    int nodes = 10;
    double mean = static_cast<double>(keys.size()) / nodes;

    printf("distribution, %d nodes / %zu keys\n", nodes, keys.size());
    for (int vnodes : {1, 10, 100, 500}) {
        Ring r;
        for (int n = 0; n < nodes; n++) r.addNode("node" + to_string(n), vnodes);

        map<string, long> load;
        for (int n = 0; n < nodes; n++) load["node" + to_string(n)] = 0;
        for (const string& k : keys) load[r.getNode(k)]++;

        double var = 0;
        long lo = static_cast<long>(keys.size());
        long hi = 0;
        for (const auto& entry : load) {
            double diff = static_cast<double>(entry.second) - mean;
            var += diff * diff;
            if (entry.second < lo) lo = entry.second;
            if (entry.second > hi) hi = entry.second;
        }

        double stddev = sqrt(var / nodes);
        printf("  vnodes=%3d  stddev=%7.1f (%5.1f%%)  min=%5ld  max=%5ld\n",
               vnodes, stddev, 100 * stddev / mean, lo, hi);
    }
    printf("\n");
}

static void movementReport() {
    vector<string> keys = makeKeys(100000);
    double total = static_cast<double>(keys.size());

    Ring r;
    for (int n = 0; n < 5; n++) r.addNode("node" + to_string(n), 150);

    vector<string> before;
    before.reserve(keys.size());
    for (const string& k : keys) before.push_back(r.getNode(k));

    r.addNode("node5", 150);

    long moved = 0;
    for (size_t i = 0; i < keys.size(); i++) {
        if (r.getNode(keys[i]) != before[i]) moved++;
    }

    long modMoved = 0;
    for (const string& k : keys) {
        if (ring_hash(k) % 5 != ring_hash(k) % 6) modMoved++;
    }

    printf("adding a 6th node to 5\n");
    printf("  consistent hashing  %6ld / %zu  (%4.1f%%)   theory 1/6 = 16.7%%\n",
           moved, keys.size(), 100 * static_cast<double>(moved) / total);
    printf("  hash(key) %% N       %6ld / %zu  (%4.1f%%)\n",
           modMoved, keys.size(), 100 * static_cast<double>(modMoved) / total);
}

int main() {
    removalTest();
    distributionReport();
    movementReport();
    return 0;
}
