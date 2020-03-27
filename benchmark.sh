#!/bin/bash
cargo build
if [ $? -ne 0 ]; then exit; fi

echo "RUST" 
time target/debug/sieve $1 > /tmp/rust 
echo
echo "PYTHON"
time python3 sieve.py $1 > /tmp/py 

diff /tmp/py /tmp/rust