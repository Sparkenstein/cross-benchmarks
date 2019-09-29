package main

import "github.com/kataras/iris"

func main() {
    app := iris.Default()
    app.Logger().SetLevel("off")
    app.Get("/", func(ctx iris.Context) {
        ctx.Text("Hello World")
    })

    app.Run(iris.Addr(":3000"))
}