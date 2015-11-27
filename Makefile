OPTFLAGS = -march=native -mtune=native -O2
CXXFLAGS += -g -Wall -Wextra -Wno-unused-parameter -std=c++11 -fPIC -Wno-unused-variable
CXXFLAGS += -I $(DEPINST)/include -I $(DEPINST)/include/libsnark -DUSE_ASM -DCURVE_ALT_BN128
LDFLAGS += -flto

DEPSRC=depsrc
DEPINST=depinst

LDLIBS += -L $(DEPINST)/lib -Wl,-rpath $(DEPINST)/lib -L . -lsnark -lgmpxx -lgmp

all:
	$(CXX) -o test.o test.cpp -c -MMD $(CXXFLAGS)
	$(CXX) -o sha256.o sha256.cpp -c -MMD $(CXXFLAGS)
	$(CXX) -o util.o util.cpp -c -MMD $(CXXFLAGS)
	$(CXX) -o test test.o sha256.o util.o $(CXXFLAGS) $(LDFLAGS) $(LDLIBS)

clean:
	$(RM) test.o test