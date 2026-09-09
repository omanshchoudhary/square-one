#include <bits/stdc++.h>
using namespace std;
#include "bloom.h"

static vector<string> makeKeys(const string& prefix, size_t n) {
    vector<string> keys;
    keys.reserve(n);
    for (size_t i = 0; i < n; i++) keys.push_back(prefix + to_string(i));
    return keys;
}

static int failures = 0;

static void check(bool ok, const string& label) {
    printf("  %-42s %s\n", label.c_str(), ok ? "OK" : "FAIL");
    if (!ok) failures++;
}

static void runTests() {
    printf("tests\n");

    size_t n = 100000;
    vector<string> keys = makeKeys("key", n);
    BloomFilter b(n, 0.01);
    for (const string& key : keys) b.insert(key);

    size_t missing = 0;
    for (const string& key : keys) if (!b.contains(key)) missing++;
    check(missing == 0, "no false negatives over 100k keys");
    check(b.count() == n, "count tracks inserts");

    BloomFilter empty(n, 0.01);
    size_t hits = 0;
    for (const string& key : keys) if (empty.contains(key)) hits++;
    check(hits == 0, "empty filter says no to everything");

    check(optimalBits(1000000, 0.01) == 9585088, "optimalBits(1e6, 0.01) == 9585088");
    check(optimalHashes(9585088, 1000000) == 7, "optimalHashes(9585088, 1e6) == 7");

    bool aligned = true;
    for (double p : {0.5, 0.1, 0.01, 0.001, 0.0001}) {
        if (optimalBits(n, p) % 64 != 0) aligned = false;
    }
    check(aligned, "optimalBits always a multiple of 64");

    check(optimalBits(0, 0.01) > 0, "guard: n = 0");
    check(optimalBits(1000, 0.0) > 0, "guard: p = 0");
    check(optimalBits(1000, 1.0) > 0, "guard: p = 1");
    check(optimalHashes(optimalBits(1000, 0.9999), 1000) >= 1, "guard: k never below 1");

    BloomFilter r(200, 3);
    check(r.bits() == 256, "BloomFilter(200, 3) rounds up to 256 bits");
    check(BloomFilter(4096, 0).hashes() == 1, "hashes clamped to at least 1");

    BloomFilter f(n, 0.01);
    check(f.fillRatio() == 0.0, "fresh filter has fill ratio 0");

    printf("\n");
}

static void fprValidation() {
    size_t n = 100000;
    vector<string> keys = makeKeys("key", n);
    vector<string> absent = makeKeys("absent", 1000000);

    printf("false positive rate, %zu keys inserted, %zu queried\n", n, absent.size());
    printf("  %-8s %10s %3s   %9s %9s   %8s %8s\n",
           "target", "bits", "k", "predicted", "measured", "fill", "theory");

    for (double p : {0.1, 0.01, 0.001, 0.0001}) {
        BloomFilter b(n, p);
        for (const string& key : keys) b.insert(key);

        size_t hits = 0;
        for (const string& key : absent) if (b.contains(key)) hits++;

        double measured = static_cast<double>(hits) / static_cast<double>(absent.size());

        printf("  %-8g %10zu %3d   %8.4f%% %8.4f%%   %8.4f %8.4f\n",
               p, b.bits(), b.hashes(), 100 * b.predictedFpr(), 100 * measured,
               b.fillRatio(), b.predictedFill());
    }
    printf("\n");
}

int main() {
    runTests();
    if (failures > 0) {
        printf("%d test(s) failed\n", failures);
        return 1;
    }
    fprValidation();
    return 0;
}
