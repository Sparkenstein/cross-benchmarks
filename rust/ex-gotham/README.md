## gotham

## debug
TL;DR debug mode `req/s: 26k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency    23.00ms   32.88ms 312.85ms   84.87%
    Req/Sec     6.63k     1.38k   12.01k    71.67%
  792263 requests in 30.01s, 125.42MB read
Requests/sec:  26395.60
Transfer/sec:      4.18MB
```

## release
TL;DR release mode `req/s: 242k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   487.62us  514.91us  23.73ms   96.85%
    Req/Sec    61.00k     3.05k   68.69k    85.75%
  7283237 requests in 30.02s, 1.13GB read
Requests/sec: 242636.26
Transfer/sec:     38.41MB
```