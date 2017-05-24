CXXFLAGS = -O3 --std=c++14

optinterp: optinterp.o parser.o utils.o
	g++ -o $@ $^ $(CXX_FLAGS)

optinerp.o: parser.h utils.h

clean:
	$(RM) optinterp *.o
