#/bin/sh -ex

( cd cpp; make )

rustup run nightly cargo build --release --features=jit

time cpp/optinterp bf/mandelbrot.bf >/dev/null
time cpp/optinterp2 bf/mandelbrot.bf >/dev/null
time target/release/bfi --peep bf/mandelbrot.bf >/dev/null
time target/release/bfi --jit bf/mandelbrot.bf >/dev/null
time target/release/bfi --jit -u bf/mandelbrot.bf >/dev/null
