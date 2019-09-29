# nickel

## debug
TL;DR debug mode `req/s: 29k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   178.18us  515.24us  20.16ms   99.50%
    Req/Sec    14.68k     8.86k   27.40k    55.17%
  876495 requests in 30.06s, 122.04MB read
Requests/sec:  29162.17
Transfer/sec:      4.06MB
```

## release
TL;DR release mode `req/s: 167k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    29.14us   95.31us  10.73ms   99.73%
    Req/Sec    84.12k    55.80k  143.76k    45.67%
  5022538 requests in 30.06s, 699.32MB read
Requests/sec: 167057.75
Transfer/sec:     23.26MB
```