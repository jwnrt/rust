# CHERIoT Rust compiler

This is an experimental fork of `rustc` adding CHERIoT support.

I hereby license all my changes under MIT and Apache-2.0 as normal; feel free to use them.
I rebase this branch frequently so make a clone if you need to pin a specific commit.

## Changes

* `usize` is address-width'd and aligned, not pointer-width'd and aligned.
* New target `riscv32cheriot-none-elf` with 32-bit addresses and 64-bit pointers.
* New target config: `#[cfg(target_address_width = ("16"|"32"|"64"))]`.
* New RISC-V target feature: `#[cfg(target_feature = "xcheri")]`.

## Status

What works?

* `core` and `alloc` compile.
* `rust-lld` can link executables.

What is definitely broken?

* `i128` is passed with an incompatible ABI for some functions.
* Atomics aren't lowered properly so are disabled (i.e. there's no `AtomicUsize`).
* `memcpy` is not yet capability aware.
* Pointer <-> `usize` casts are still possible (`fuzzy_provenance_casts` is not yet forbid-by-default).
* `global_asm!` does not have CHERI target features in release/thin-lto builds.
  * This is an LLVM bug: [llvm-project#61991][llvm-bug-asm].
  * Work around it by enabling the arch options yourself in the ASM:
    ```rust
    core::arch::global_asm! { r#"
      .option push
      .option arch, +xcheri
      .option capmode

      csetaddr csp, ca5, a0
      ccall main

      .option pop
    "# }
    ```

What is still unknown?

* Basically everything. I haven't executed a single generated instruction yet.

[llvm-bug-asm]: https://github.com/llvm/llvm-project/issues/61991

## Building

Follow the regular `rustc` build instructions: [how to build and run the compiler][build-and-run].

Here's an example `bootstrap.toml`:

```toml
profile = "compiler"

[llvm]
download-ci-llvm = false
targets = "RISCV;X86"
experimental-targets = ""
release-debuginfo = true
assertions = true

[rust]
lld = true

[build]
target = [
  "x86_64-unknown-linux-gnu",
  "riscv32cheriot-none-elf",
  # "riscv32imc-unknown-none-elf",
]
```

Important notes:

1. This builds LLVM from source using the CHERIoT-LLVM submodule at `src/llvm-project`.
2. Read [these instructions][prebuilt-llvm] if you want to use a pre-built CHERIoT-LLVM toolchain.
   Remember to set `cc = "path"` and friends as well. Keep `download-ci-llvm = false`.
3. The host toolchain is required for staged builds.
4. Enable the `riscv32imc` target to test the compiler with vanilla Ibex.

[build-and-run]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html
[prebuilt-llvm]: https://rustc-dev-guide.rust-lang.org/building/new-target.html#using-pre-built-llvm
