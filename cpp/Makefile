CXXFLAGS = -O3 --std=c++11
EXE      = optasmjit optinterp3 optinterp2 optinterp

default: $(EXE)

asmjit:
	../scripts/build_asmjit.sh

optasmjit: optasmjit.o parser.o utils.o
	$(MAKE) asmjit
	$(CXX) -o $@ $^ -Lasmjit/build -lasmjit

optinterp3: optinterp3.o parser.o utils.o
	$(CXX) -o $@ $^

optinterp2: optinterp2.o parser.o utils.o
	$(CXX) -o $@ $^

optinterp: optinterp.o parser.o utils.o
	$(CXX) -o $@ $^

optasmjit.o: optasmjit.cpp
	$(MAKE) asmjit
	$(CXX) -c $< $(CXXFLAGS) -Iasmjit/src

optasmjit.o optinerp3.o optinerp2.o optinerp.o: parser.h utils.h

clean:
	$(RM) $(EXE) *.o
