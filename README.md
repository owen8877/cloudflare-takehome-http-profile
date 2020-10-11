# HTTP Profiler
## Build
First make sure you have rust toolchain. Then run the following command at the root directory:
```bash
cargo run
```
You shall see the help hints of the program:
```
Usage: target/debug/http-profiler [options]

Options:
    -u, --url https://...
                        the target to fetch
    -p, --profile [int] the number of requests
    -h, --help          print this help menu
```

To run the profiler:
1. use `--` as separator between `cargo run` and the options (see examples down below)
2. option `-u` is to indicate the target website (mandatory)
3. option `-p` is to designate how many requests are sent (optional)

## Test run
### Single request to worker site
```
$ cargo run -- -u https://linktree-style-website.xdroid.workers.dev/
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/http-profiler -u 'https://linktree-style-website.xdroid.workers.dev/'`
Summary:
Requests made:
    Total    1
    Success  1
    Ratio    100%
Size(bytes) per request:
    Smallest 2173
    Largest  2173
Time(ms) per request:
    Fastest  350
    Mean     350
    Median   350
    Slowest  350
Error code encountered: []
```

### Multiple requests to worker site
```
$ cargo run -- -u https://linktree-style-website.xdroid.workers.dev/ -p 10
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/http-profiler -u 'https://linktree-style-website.xdroid.workers.dev/' -p 10`
Summary:
Requests made:
    Total    10
    Success  10
    Ratio    100%
Size(bytes) per request:
    Smallest 2173
    Largest  2173
Time(ms) per request:
    Fastest  146
    Mean     172
    Median   161
    Slowest  240
Error code encountered: []
```

### Multiple requests to google home page
```
$ cargo run -- -u https://www.google.com/ -p 10                           
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/http-profiler -u 'https://www.google.com/' -p 10`
Summary:
Requests made:
    Total    10
    Success  10
    Ratio    100%
Size(bytes) per request:
    Smallest 46977
    Largest  47828
Time(ms) per request:
    Fastest  112
    Mean     134
    Median   130
    Slowest  203
Error code encountered: []
```

### Multiple requests to non-existing page
```
$ cargo run -- -u https://doc.rust-lang.org/edition-guide/introduction2.html -p 10
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/http-profiler -u 'https://doc.rust-lang.org/edition-guide/introduction2.html' -p 10`
Summary:
Requests made:
    Total    10
    Success  0
    Ratio    0%
Size(bytes) per request:
    Smallest 4288
    Largest  4288
Time(ms) per request:
    Fastest  100
    Mean     139
    Median   108
    Slowest  390
Error code encountered: [404]
```