#pragma once
#include <cstdint>
#include <string>
#include <vector>
#include "hash.h"

class BloomFilter {
public:
    BloomFilter(size_t bits, int hashes){
        m=bits; // total size of the bit array
        k=hashes; // number of hashes 
        words.assign((bits + 63) / 64, 0ULL);
        inserted=0;
    }

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
    double fillRatio() const;
    double predictedFpr() const;

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