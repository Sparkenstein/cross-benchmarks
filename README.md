# cross-benchmarks

This is not a tool. it's an informative repository to compare frameworks from multiple languages. I only pick the code written by author in respective framework's `example` section usually, hello-world and test it directly. this is not a test to push frameworks to their limits, rather, it's a test for how frameworks behave in their default settings.

## Specifications
 - Manjaro on Linux 5.2.11-1-MANJARO x86_64 kernel
 - Intel(R) Core(TM) i5-4690 CPU @ 3.50GHz
 - Corsair 8GB 1600 MHz DDR3 RAM
 - Respective language compiler version under their `README`
 - wrk self compiled `v45f3fa0`

## Winners so far according to language

### Rust

#### :fire: actix-web :fire:
actix-web is a beast with an undisputed record of `327k req/s` and a transfer of `40.25 MB/s` :open_mouth: making it fastest framework not only in Rust but also compared to other languages.



## Todo
 - Languages to add
    - Clojure
	- Nim
	- Scala?
	- Crystal
	- C/C++ (needs compilation for everything, might drop the plan!)
	- Elixir
 - Frameworks to add
    - kemal (crystal)
