package api

import (
	"github.com/gin-gonic/gin"
)

func Serve(addr string, startbanner bool, debug bool) {
	r := gin.Default()
	r.Run(addr)
}
