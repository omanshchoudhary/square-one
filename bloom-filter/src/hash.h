#pragma once
#include <cstdint>
#include <string>

inline uint64_t mix64(uint64_t h) {
    h ^= h >> 30;
    h *= 0xbf58476d1ce4e5b9ULL;
    h ^= h >> 27;
    h *= 0x94d049bb133111ebULL;
    h ^= h >> 31;
    return h;
}

inline uint64_t fnv1a(const std::string& key, uint64_t basis) {
    const uint64_t prime = 1099511628211ULL;
    uint64_t h = basis;
    for (char c : key) {
        h ^= static_cast<uint8_t>(c);
        h *= prime;
    }
    return h;
}

inline uint64_t hash1(const std::string& key) {
    return mix64(fnv1a(key, 14695981039346656037ULL));
}

inline uint64_t hash2(const std::string& key) {
    // odd so h2 is never a multiple of m
    return mix64(fnv1a(key, 0x9e3779b97f4a7c15ULL)) | 1ULL;
}
