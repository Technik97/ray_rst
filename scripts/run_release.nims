#!/usr/bin/env -S nim e --hints:off

echo "Starting bin in Release mode"
exec "cargo run --release  > out.ppm"