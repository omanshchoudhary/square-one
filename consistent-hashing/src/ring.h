#pragma once
#include "hash.h"
#include<cstdint>
#include<map>
#include<string>

class Ring {
public:
    void addNode(const std::string& name, int vnodes){
        nodes[name] = vnodes;
        for(int i=0;i<vnodes;i++){
            std::string vKey = vnodeKey(name,i);
            uint64_t hash = ring_hash(vKey);
            ring[hash]=name;
        }
    }
    void removeNode(const std::string& name){
        auto it = nodes.find(name);
        if(it==nodes.end()) return ;
        int vnodes = it->second;
        for(int i=0;i<vnodes;i++){
            std::string vKey = vnodeKey(name,i);
            uint64_t hash = ring_hash(vKey);
            ring.erase(hash);
        }
        nodes.erase(it);
        return;
    }
    std::string getNode(const std::string& key) const{
        if(empty()) return "";
        uint64_t hash = ring_hash(key);
        auto it = ring.lower_bound(hash);
        if(it==ring.end()) it= ring.begin();
        return it->second;
    }

    bool empty() const {
        return ring.empty();
    }
    size_t size() const {
        return ring.size();
    }

private:
    std::map<uint64_t,std::string> ring;
    std::map<std::string, int> nodes;

    static std::string vnodeKey(const std::string& name, int i){
        return name + "$" + std::to_string(i);
    }
};