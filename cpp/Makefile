CXXFLAGS = -O3 --std=c++14

default: optinterp2 optinterp

optinterp2: optinterp2.o parser.o utils.o
	g++ -o $@ $^ $(CXX_FLAGS)

optinterp: optinterp.o parser.o utils.o
	g++ -o $@ $^ $(CXX_FLAGS)

optinerp2.o: parser.h utils.h
optinerp.o: parser.h utils.h

clean:
	$(RM) optinterp *.o
