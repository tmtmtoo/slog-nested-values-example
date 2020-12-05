```bash
$ RUST_LOG=info cargo run
   Compiling slog-nested-values-example v0.1.0 (/Users/matsumototakahiro/rust-projects/slog-nested-values-example)
    Finished dev [unoptimized + debuginfo] target(s) in 1.84s
     Running `target/debug/slog-nested-values-example`
{"msg":"JSON list","level":"INFO","ts":"2020-12-05T23:52:25.731280+09:00","version":"0.1.0","list":[123,456]}
{"msg":"JSON object","level":"ERRO","ts":"2020-12-05T23:52:25.731921+09:00","version":"0.1.0","object":{"x":1,"y":"test","z":{"v":"masking is possible!!!!"}}}
```

```bash
$ RUST_LOG=error cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/slog-nested-values-example`
{"msg":"JSON object","level":"ERRO","ts":"2020-12-05T23:52:51.757006+09:00","version":"0.1.0","object":{"x":1,"y":"test","z":{"v":"masking is possible!!!!"}}}
```
