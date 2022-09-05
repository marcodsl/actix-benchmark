# Actix Benchmarks

## Abstract

The purpose of this benchmark is to test whether using Dynamic Dispatch, instead of Static Dispatch, exclusively on the dependencies of a Handler impacts the performance of an application considerably.

## Benchmarks

### Static Dispatch

```powershell
$ wrk -t12 -c400 -d30s http://127.0.0.1:8080/primes

Running 30s test @ http://127.0.0.1:8080/primes
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   195.43ms   18.57ms 422.88ms   78.14%
    Req/Sec   102.81     39.24   280.00     66.58%
  36927 requests in 30.10s, 21.22GB read
  Socket errors: connect 155, read 242, write 0, timeout 0
Requests/sec:   1226.96
Transfer/sec:    722.02MB
```

### Dynamic Dispatch

```powershell
$ wrk -t12 -c400 -d30s http://127.0.0.1:8080/primes

Running 30s test @ http://127.0.0.1:8080/primes
  12 threads and 400 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   197.47ms   21.05ms 423.41ms   78.12%
    Req/Sec   102.00     47.90   270.00     61.69%
  36515 requests in 30.09s, 20.98GB read
  Socket errors: connect 155, read 258, write 0, timeout 0
Requests/sec:   1213.40
Transfer/sec:    714.04MB
```

## Conclusion

Although Dynamic Dispatch has a lower performance compared to Static Dispatch, this difference is minute and does not negatively impact the overall performance of the application.
