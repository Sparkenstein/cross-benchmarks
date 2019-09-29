package main

import (
	"flag"
	"fmt"
	"github.com/valyala/fasthttp"
)

var addr = flag.String("addr", ":3000", "TCP address to listen to")

func main() {
	flag.Parse()

	h := requestHandler
	fasthttp.ListenAndServe(*addr, h)
}

func requestHandler(ctx *fasthttp.RequestCtx) {
	fmt.Fprintf(ctx, "Hello, world")
}