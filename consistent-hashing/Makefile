CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -O2
SRC = $(wildcard src/*.cpp)

ring: $(SRC)
	$(CXX) $(CXXFLAGS) -o $@ $(SRC)

clean:
	rm -f ring

.PHONY: clean
