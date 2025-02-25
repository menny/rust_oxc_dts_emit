# oxc_dts_emit
Rust wrapper for [oxc dts emitter](https://crates.io/crates/oxc).

## How to run
Using Bazel:
```shell
bazel run :oxc_dts_emit -- [/path/to/ts/file.ts:/path/to/ts/file.d.ts] [/path/to/ts/file2.ts:/path/to/ts/file2.d.ts]
```
You can add as many [input_path:output_path] as you wish.

If you are using the released binary, you can run it similarly:
```shell
./oxc_dts_emit -- [/path/to/ts/file.ts:/path/to/ts/file.d.ts] [/path/to/ts/file2.ts:/path/to/ts/file2.d.ts]
```
