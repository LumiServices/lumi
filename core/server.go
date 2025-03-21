package core

//not sure what I'll do with this

import (
	"net/http"

	"github.com/gin-gonic/gin"
)

func CreateHTTP(addr string) {
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{
			"message": "pong",
		})
	})
	r.Run(addr)
}
