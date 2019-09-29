# Rust

Rust is a multi-paradigm system programming language focused on safety, especially safe concurrency. Rust is syntactically similar to C++, but is designed to provide better memory safety while maintaining high performance [Wiki]

## Specifications
 - [nightly] rustc `1.40.0-nightly` (488381ce9 2019-09-28)
 - [stable] rustc `1.38.0` (625451e37 2019-09-23)

## frameworks used
 - actix-web
 - gotham
 - hyper
 - tokio-minhttp

## Result
| Framework       | total reqs | req/s     | read size | transfer/s | latency (avg) |
| -----------     | ---------- | ------    | --------- | ---------- |---------------|
| actix-web 🌟     | 9822571    | 327190.16 | 1.18GB    | 40.25MB    | 728.46us      |
| gotham          | 7283237    | 242636.26 | 1.13GB    | 38.41MB    | 487.62us      |
| hyper           | 8946802    | 297978.85 | 759.38MB  | 25.29MB    | 636.79us      |
| tokio-minhttp   | 4762521    | 158615.27 | 467.82MB  | 15.58MB    | 767.68us      |
| iron            | 4788528    | 159160.97 | 516.04MB  | 17.15MB    | 277.82us      |
| nickel          | 5022538    | 167057.75 | 699.32MB  | 23.26MB    | 29.14us       |
| rocket          | 2047514    | 68029.47  | 285.09MB  | 9.47MB     | 1.24ms        |
| tiny-http       | 3769158    | 125517.76 | 553.56MB  | 18.43MB    | 0.96ms        |
