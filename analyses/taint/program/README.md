# Example Dynamic Taint Tracking with Wasabi

Rough steps:

- Take a C program that does something interesting with inputs
    * E.g. here: "encrypt" an input string and return the output as a `char *`
- Conceptual work: Define a taint policy
    * What is a taint?
        * just single bit: 0 = not tainted, 1 = tainted
        * tracking the input: "this result was obtained from bytes N, M, ... in the input"
        * more complex taints: collect the whole expression from which this value was derived (e.g., for dynamic symbolic execution)
    * What are "inputs", when are they tainted?
        * For C programs compiled with emscripten, we say "any write to the WebAssembly memory in the emscripten-generated JavaScript code" is tainted
        * other options could be: return values from imported functions in WebAssembly are always tainted
    * How is taint propagated?
        * for boolean taints: "if one of the inputs of a binary operation is tainted, its output is also tainted"
        * track taints through function arguments, locals, globals, operand stack, memory loads/stores
        * are results of conditionals/comparison operators tainted? might lead to very large taint creep
    * What is a sink?
        * here: no explicit sink, just look at memory after execution finished, where did the taint flow?
- Implementation:
    1. Compile C to wasm.
    2. Write taint analysis against Wasabi API in JS.
    3. Modify emscripten-generated JS code to introduce taint on memory writes.
    4. Instrument wasm program with Wasabi + add wasabi.js and analysis.js to html harness
- Run program with analysis code
    * check where did taints flow

## In `original/`:

- `enc.c`: Simple C "library" that contains a single `enc`ryption function.
- `build.sh`: Build instructions.
- `enc.html`: Simple HTML harness that makes use of the C code (once it is compiled to WebAssembly).

## In `wasabi/`:

- `instrument.sh`: After copying over the files from `original/`, instrument binary with this.
- `enc.js`: Modified from the emscripten-generated JS code in two ways:
    * used https://beautifier.io/ to make more readable
    * look for all writes to the WebAssembly.Memory object
        - `WebAssembly.Memory.buffer` gives you an ArrayBuffer
        - `new Int8Array(buffer)` gives you a TypedArray view into that buffer (which you can write to)
        - Emscripten has some abbreviations for those typed views, e.g., `HEAP8`, `HEAPU32`, etc.
        - look for writes through those abbreviations, e.g., `HEAP8[someOffset] = someValue` or `HEAP8.set(arrayOfValues, someOffset)`
    * whenever something writes to the concrete WebAssembly memory, also write taint to `Wasabi.taintMemory[someOffset] = new Taint(1)` (see `taint.js`)
- `begin_end_matches.js`: Just a very simple analysis to make sure that begin and end hooks are always matching (has nothing to do with taint analysis really, was just a sanity check for me).
- `taint.js`: Quick and dirty taint analysis that tracks taints through the stack, function calls, memory loads and stores etc. Based on taint analysis in `/analyses/taint.js`

Open `enc.html` in Firefox, run program, open DevTools console, write `Wasabi.taintMemoryPrint()` and see that we successfully tracked the tainted input values through the "encryption" operation to the memory.
