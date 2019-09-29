# actix-web

## debug
TL;DR debug mode `req/s: 40k` 
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.98ms  594.99us  33.13ms   93.90%
    Req/Sec    10.16k   758.97    22.92k    84.86%
  1215522 requests in 30.10s, 149.54MB read
Requests/sec:  40388.21
Transfer/sec:      4.97MB
```

## release
TL;DR release mode `req/s: 327k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   728.46us    1.86ms  28.03ms   94.68%
    Req/Sec    82.24k    11.09k  107.05k    65.75%
  9822571 requests in 30.02s, 1.18GB read
Requests/sec: 327190.16
Transfer/sec:     40.25MB
```