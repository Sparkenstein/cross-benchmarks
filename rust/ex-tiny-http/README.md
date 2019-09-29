# tiny-http

## debug
TL;DR debug mode `req/s: 13k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     8.74ms  337.55us  33.35ms   90.96%
    Req/Sec     3.45k    84.75     3.61k    68.25%
  412039 requests in 30.04s, 60.51MB read
Requests/sec:  13716.26
Transfer/sec:      2.01MB
```

## release 
TL;DR debug mode `req/s: 125k` ( CPU usage was quiet low with this one. )
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     0.96ms  289.74us  19.60ms   96.51%
    Req/Sec    31.57k     2.15k   36.47k    90.83%
  3769158 requests in 30.03s, 553.56MB read
Requests/sec: 125517.76
Transfer/sec:     18.43MB
```