CXX = g++
CXXFLAGS = -std=c++17 -Wall -Wextra -Wconversion -O2
SRC = $(wildcard src/*.cpp)

bloom: $(SRC)
	$(CXX) $(CXXFLAGS) -o $@ $(SRC)

clean:
	rm -f bloom

.PHONY: clean
