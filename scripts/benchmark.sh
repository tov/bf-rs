#/bin/sh -x

BIG_PRIME=179424691

make -C cpp

rustup run nightly cargo build --release --features=jit

heading () {
    echo "$1"
    echo "$1" | sed 's/./-/g'
}

time_cmd () {
    input="$1"
    shift
    if [ -n "$input" ]; then
        echo "\$ echo "$input" | $@"
        time echo "$input" | $@ > /dev/null
    else
        echo "\$ $@"
        time $@ > /dev/null
    fi
    echo
}

bench () {
    heading "$1"
    shift
    time_cmd "" "$@" bf/mandelbrot.bf
    time_cmd "" "$@" bf/mandelbrot-quiet.bf
    time_cmd "$BIG_PRIME" "$@" bf/factor.bf
}

bench "bfi peephole AST"                target/release/bfi --peep
bench "bfi bytecode"                    target/release/bfi --flat
bench "bfi native JIT"                  target/release/bfi --jit
bench "bfi native JIT (unchecked)"      target/release/bfi --jit -u
bench "Bendersky's optinterp3"          cpp/optinterp3
bench "Bendersky's optasmjit"           cpp/optasmjit

exit 0
