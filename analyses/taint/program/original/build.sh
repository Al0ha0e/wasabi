#!/bin/sh
emcc -O3 -s EXTRA_EXPORTED_RUNTIME_METHODS='["ccall"]' enc.c -o enc.js
