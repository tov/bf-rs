#/bin/sh -x

BIG_PRIME=179424691

make -C cpp

rustup run nightly cargo build --release --features=jit

if ! which bfc >/dev/null 2>&1; then
    if [ -d /usr/local/opt/llvm/ ]; then
        LLVM_SYS_40_PREFIX=/usr/local/opt/llvm/
        export LLVM_SYS_40_PREFIX
    else
        LLVM_SYS_40_AUTOBUILD=1
        export LLVM_SYS_40_AUTOBUILD
    fi

    cargo install --git https://github.com/tov/bfc.git
fi

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
bench "bfc compilation"                 bfc

heading "bfc execution"
time_cmd ""             ./mandelbrot
time_cmd ""             ./mandelbrot-quiet
time_cmd "$BIG_PRIME"   ./factor

exit 0
