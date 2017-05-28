CXXFLAGS = -O3 --std=c++11

default: optinterp3 optinterp2 optinterp

optinterp3: optinterp3.o parser.o utils.o
	g++ -o $@ $^ $(CXX_FLAGS)

optinterp2: optinterp2.o parser.o utils.o
	g++ -o $@ $^ $(CXX_FLAGS)

optinterp: optinterp.o parser.o utils.o
	g++ -o $@ $^ $(CXX_FLAGS)

optinerp3.o: parser.h utils.h
optinerp2.o: parser.h utils.h
optinerp.o: parser.h utils.h

clean:
	$(RM) optinterp *.o
