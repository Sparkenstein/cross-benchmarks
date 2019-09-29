# rocket

## debug
TL;DR debug mode `req/s: 16k`
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.10ms    1.58ms  40.76ms   91.10%
    Req/Sec     4.17k   146.05     4.73k    75.33%
  498234 requests in 30.03s, 69.37MB read
  Socket errors: connect 0, read 498231, write 0, timeout 0
Requests/sec:  16591.46
Transfer/sec:      2.31MB
```

## release
TL;DR debug mode `req/s: 68k` ( I Expected a REAL large number! maybe I misconfigured something :( )
```
Running 30s test @ http://localhost:3000/
  4 threads and 120 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.24ms    1.05ms  23.96ms   86.26%
    Req/Sec    17.14k     1.50k   22.09k    73.46%
  2047514 requests in 30.10s, 285.09MB read
  Socket errors: connect 0, read 2047498, write 0, timeout 0
Requests/sec:  68029.47
Transfer/sec:      9.47MB
```