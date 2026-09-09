#pragma once
#include <string>
#include <cstdint>


inline uint64_t fnv1a(const std::string& key){
    uint64_t hash = 14695981039346656037ULL;
    const uint64_t prime = 1099511628211ULL;

    for(char c: key) {
        hash^= static_cast<uint8_t>(c); // xor the bottom 8 bits
        // then multiply by the prime
        hash*=prime;
    }
    return hash;
}

inline uint64_t ring_hash(const std::string& key) {
    uint64_t h = fnv1a(key);

    h ^= h >> 30;
    h *= 0xbf58476d1ce4e5b9ULL;
    h ^= h >> 27;
    h *= 0x94d049bb133111ebULL;
    h ^= h >> 31;
    
    return h;
}