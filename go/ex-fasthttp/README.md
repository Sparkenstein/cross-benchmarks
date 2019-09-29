# fasthttp

TL;DR `req/s: 311k` (almost beat Rust O.o)
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   836.23us    2.55ms 100.85ms   95.38%
    Req/Sec    78.21k     8.60k  132.03k    79.92%
  9344105 requests in 30.04s, 1.28GB read
Requests/sec: 311030.85
Transfer/sec:     43.60MB
```