# iron

## debug
TL;DR debug mode `req/s: 18k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.82ms    6.44ms 113.54ms   90.40%
    Req/Sec     6.05k     2.20k   10.11k    56.89%
  542092 requests in 30.05s, 58.42MB read
Requests/sec:  18037.11
Transfer/sec:      1.94MB
```

## release
TL;DR release mode `req/s: 159k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   277.82us    1.58ms 118.16ms   98.86%
    Req/Sec    53.45k    25.22k  107.75k    42.56%
  4788528 requests in 30.09s, 516.04MB read
Requests/sec: 159160.97
Transfer/sec:     17.15MB
```