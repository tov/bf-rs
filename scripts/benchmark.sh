#/bin/sh -x

BIG_PRIME=179424691

( cd cpp; make )

rustup run nightly cargo build --release --features=jit

bench () {
    echo "$1"
    echo "$1" | sed 's/./-/g'
    shift
    echo "\$ $@ bf/mandelbrot.bf"
    time "$@" bf/mandelbrot.bf > /dev/null
    echo
    echo "\$ $@ bf/mandelbrot-quiet.bf"
    time "$@" bf/mandelbrot-quiet.bf > /dev/null
    echo
    echo "\$ echo $BIG_PRIME | $@ bf/factor.bf"
    time echo $BIG_PRIME | "$@" bf/factor.bf > /dev/null
    echo
}

bench "bfi peephole AST" target/release/bfi --peep
bench "bfi bytecode" target/release/bfi --flat
bench "bfi native JIT" target/release/bfi --jit
bench "bfi native JIT (unchecked)" target/release/bfi --jit -u
# bench "Bendersky's optinterp" cpp/optinterp
# bench "Bendersky's optinterp2" cpp/optinterp2
bench "Bendersky's optinterp3" cpp/optinterp3
bench "Bendersky's optasmjit" cpp/optasmjit

exit 0
