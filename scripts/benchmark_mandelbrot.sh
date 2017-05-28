#/bin/sh -x

( cd cpp; make )

rustup run nightly cargo build --release --features=jit

bench () {
    echo "$1"
    echo "$1" | sed 's/./-/g'
    shift
    echo "\$ $@"
    time "$@" > /dev/null
    echo
}

bench "bfi peephole AST" target/release/bfi --peep bf/mandelbrot.bf
bench "bfi bytecode" target/release/bfi --flat bf/mandelbrot.bf
bench "bfi native JIT" target/release/bfi --jit bf/mandelbrot.bf
bench "bfi native JIT (unchecked)" target/release/bfi --jit -u bf/mandelbrot.bf
# bench "Bendersky's optinterp" cpp/optinterp bf/mandelbrot.bf
# bench "Bendersky's optinterp2" cpp/optinterp2 bf/mandelbrot.bf
bench "Bendersky's optinterp3" cpp/optinterp3 bf/mandelbrot.bf
bench "Bendersky's optasmjit" cpp/optasmjit bf/mandelbrot.bf

exit 0
