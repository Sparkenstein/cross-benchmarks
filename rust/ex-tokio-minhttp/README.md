# tokio-minhttp

## debug
TL;DR debug mode `req/s: 21k`
```
Running 30s test @ http://localhost:8080/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.66ms  623.53us  41.83ms   99.40%
    Req/Sec     5.33k   177.18     5.74k    78.08%
  637167 requests in 30.05s, 62.59MB read
Requests/sec:  21206.41
Transfer/sec:      2.08MB
```

## release
TL;DR release mode `req/s: 158k`

```
Running 30s test @ http://localhost:8080/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   767.68us  495.34us  30.60ms   99.66%
    Req/Sec    39.88k     1.58k   46.58k    86.17%
  4762521 requests in 30.03s, 467.82MB read
Requests/sec: 158615.27
Transfer/sec:     15.58MB
```