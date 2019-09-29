# Hyper

## debug
TL;DR debug mode `req/s: 55k`

```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.48ms   11.61ms  94.85ms   83.57%
    Req/Sec    14.05k     2.26k   22.57k    64.83%
  1677894 requests in 30.02s, 142.41MB read
Requests/sec:  55885.71
Transfer/sec:      4.74MB
```

## release
TL;DR release mode `req/s: 297k` :astonished:
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   636.79us    1.17ms  24.32ms   93.24%
    Req/Sec    74.89k    12.72k  124.00k    78.00%
  8946802 requests in 30.02s, 759.38MB read
Requests/sec: 297978.85
Transfer/sec:     25.29MB
```