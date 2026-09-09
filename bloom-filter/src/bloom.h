#pragma once
#include <cmath>
#include <cstdint>
#include <string>
#include <vector>
#include "hash.h"

// n = items you expect to insert
// p = false positive rate you want
inline size_t optimalBits(size_t n, double p) {
    if (n == 0 || p <= 0.0 || p >= 1.0) return 64;
    const double ln2 = std::log(2.0);
    double raw = -(static_cast<double>(n) * std::log(p)) / (ln2 * ln2);
    size_t bits = static_cast<size_t>(std::ceil(raw));
    return ((bits + 63) / 64) * 64;
}

inline int optimalHashes(size_t m, size_t n) {
    if (n == 0) return 1;
    const double ln2 = std::log(2.0);
    double raw = (static_cast<double>(m) / static_cast<double>(n)) * ln2;
    int k = static_cast<int>(std::round(raw));
    return k < 1 ? 1 : k;
}

class BloomFilter {
public:
    BloomFilter(size_t bits, int hashes){
        m=((bits + 63) / 64) * 64; // total size of the bit array
        k=hashes < 1 ? 1 : hashes; // number of hashes
        words.assign(m / 64, 0ULL);
        inserted=0;
    }

    BloomFilter(size_t expectedItems, double targetFpr)
        : BloomFilter(optimalBits(expectedItems, targetFpr),
                      optimalHashes(optimalBits(expectedItems, targetFpr), expectedItems)) {}

    void insert(const std::string& key) {
        uint64_t h1 = hash1(key);
        uint64_t h2 = hash2(key);

        for(int i=0;i<k;i++){
            size_t bit = (h1+i*h2)%m;
            setBit(bit);
        }
        inserted++;
    }
    bool contains(const std::string& key) const {
        uint64_t h1 = hash1(key);
        uint64_t h2 = hash2(key);

        for(int i=0;i<k;i++){
            size_t bit = (h1+i*h2)%m;
            if(!testBit(bit)) return false;
        }
        return true;
    }

    size_t bits() const {
        return m;
    }
    int hashes() const {
        return k;
    }
    size_t count() const {
        return inserted;
    }
    // counts bits actually set
    double fillRatio() const {
        size_t set = 0;
        for (uint64_t w : words) set += static_cast<size_t>(__builtin_popcountll(w));
        return static_cast<double>(set) / static_cast<double>(m);
    }
    // theoretical with the help of formula
    double predictedFpr() const {
        double exponent = -(static_cast<double>(k) * static_cast<double>(inserted))
                          / static_cast<double>(m);
        return std::pow(1.0 - std::exp(exponent), static_cast<double>(k));
    }

private:
    std::vector<uint64_t> words;
    size_t m;
    int k;
    size_t inserted;

    void setBit(size_t i) {
        size_t word_index = i/64;
        size_t bit_offset = i%64;

        words[word_index] |= (1ULL << bit_offset);
    }
    bool testBit(size_t i) const {
        size_t word_index = i / 64;
        size_t bit_offset = i % 64;

        return (words[word_index] & (1ULL << bit_offset)) != 0;
    }
};