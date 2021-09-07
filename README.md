Here lies the tinkering of how to use `deno_runtime` as a dependency to wrap the [Deno](https://deno.land/) runtime in a custom Rust program as an alternative to using the Deno CLI (`deno`) binary. 

I find it fascinating that you can pre-resolve remote modules, pre-compile TS and bundle untrusted JS into a single file and then execute that somewhere that doesn't need access to any (or only very specific) system resources.

## Building and running the new Rust binary

Build produces `./target/debug/theno` -- you get it? As-in thin Deno... oh, well, nevermind.
```
cargo build -vv
```

`theno` takes one argument that is a path to a javascript source file. 

There are a few JS files in `samples` that can be tried. For example:

```
$./target/debug/theno samples/ok_console.js
This is awaited now 1630961150330
```

```
 $./target/debug/theno samples/ok_fetch.js
Similicaudipteryx Lametasaurus Klamelisaurus.
```

```
$./target/debug/theno samples/err_compile.js
error: ReferenceError: foo is not defined
    at file:///Users/mbrevoort/dev/deno/theno/samples/err_compile.js:1:1
```

```
$./target/debug/theno samples/err_throw.js
error: Error: an error has been thrown
    at file:///Users/mbrevoort/dev/deno/theno/samples/err_throw.js:1:7
```

```
$./target/debug/theno samples/err_fetch.js
error: Error: Requires net access to "unallowed.com", run again with the --allow-net flag
    at deno:core/01_core.js:106:46
    at unwrapOpResult (deno:core/01_core.js:126:13)
    at Object.opSync (deno:core/01_core.js:140:12)
    at opFetch (deno:ext/fetch/26_fetch.js:57:17)
    at mainFetch (deno:ext/fetch/26_fetch.js:199:61)
    at deno:ext/fetch/26_fetch.js:439:11
    at new Promise (<anonymous>)
    at fetch (deno:ext/fetch/26_fetch.js:399:15)
    at file:///Users/mbrevoort/dev/deno/theno/samples/err_fetch.js:1:20
```

## Divergent exploration, Go bindings for Rust Library

After [this question](https://twitter.com/progrium/status/1435008458525397005), I wanted to learn how to bind to a Rust library from Go. 

I was able to get a dynamically linked version working but the staticly compiled version was failing failing on my M1 Mac because of a linking issue. 

`godynotheno.go` uses cgo to bind to the Rust library defined in `src/lib.rs`. To try it out:
1. `make build-dynamic`
2. Run the Go binary, that binds `run()` in the Rust library and executes the javascript source in `deno_runtime`:
```
$./bin/godynotheno samples/ok_fetch.js
Dracorex Qiupalong Chuandongocoelurus.
```