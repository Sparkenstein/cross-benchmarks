package main

import "github.com/gin-gonic/gin"

func main() {
	r := gin.Default()

	r.GET("/ping", func(c *gin.Context) {
		c.String(200, "Hello World")
	})
	r.Run(":3000") // listen and serve on 0.0.0.0:8080
}