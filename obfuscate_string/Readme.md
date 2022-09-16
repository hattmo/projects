# Simple String Obfuscator

## Overview

Simple string obfuscator attempts to remove string literals from the resulting binary by
creating a pair of u8 blobs that when xor'd generate the orginal keys.  In the code
when a string is referenced instead of pointing to a literal a call is made to the
deobfuscator which build the string back in the heap and returns a pointer.  the function
is reentrant and only allocates the memory once so the result at runtime is pretty much
the same except theres a function call in between and the string is in the heap instead
of some other RO section or elsewhere.

## Building

Make sure you clone the submodule that contains the tree-sitter parser.  then simply
`cargo build` or `cargo build -r`

## Usage

```bash
$ simple_string_obfuscator <INPUT> <OUTPUT>
```

## Notes

Simple string obfuscator will not obfuscate string that a global or const defined.
Additionally it will not parse escape codes (work in progress).
