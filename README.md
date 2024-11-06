# ProVerif Terminator

[![Cargo](https://github.com/famoser/proverif-terminator/actions/workflows/build.yml/badge.svg)](https://github.com/famoser/proverif-terminator/actions/workflows/build.yml)

Pipe the output of ProVerif (with `set verboseRules = true.`) into `proverif-terminator`, and enjoy some condensed information, which (may) help to debug non-termination issues.

Features (in selected facts):
- Print selected facts condensed
- Detect high counter
- Detect loops
