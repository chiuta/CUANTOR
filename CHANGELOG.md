# Changelog

## 0.2.1

### Added

- Interface in 8 languages (GUI with a language selector, CLI help and errors).
- Adversarial test suite: parser fuzzing, resource bombs, engine cross-checks, statistics, hostile REST responses.
- IBM Quantum integration: transpiler to native gates with routing, Runtime REST client, Composer export.
- Presentation website and documentation in 8 languages.

### Fixed

- Exponential expansion of nested user-defined gates could allocate gigabytes and abort the process; expansion is now bounded.
- Deeply nested expressions overflowed the stack and aborted the process; nesting depth is now limited to 128.
- Sampling results depended on the number of threads; results now depend only on the seed.
- The GUI started an unbounded number of analysis threads under rapid edits (memory exhaustion at 4096 qubits); there is now one worker plus one queued request.
- Invalid noise probabilities (above 1, negative, NaN) were silently accepted; they are now rejected.
- The CLI silently ignored invalid numeric options such as --shots abc and unknown engines; they are now errors.
- Several symbols were missing from the default font and rendered as empty boxes; replaced with text labels.

### Notes

- Not verified on real IBM hardware.
- Dynamic circuits cannot be drawn in the editor or sent to IBM hardware.
- Example titles remain in Romanian.
