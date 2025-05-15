package api

import (
	"fmt"

	"github.com/gin-gonic/gin"
	"github.com/ros-e/lumi/s3"
)

func Serve(addr string, startbanner bool, debug bool) {
	if debug {
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	fmt.Printf("[\033[35m REST API started on \033[0mhttp://%s ]\n", addr)
	r := gin.Default()
	r.GET("/:bucket/*key", GetObject)
	r.GET("/:bucket", s3.S3AuthMiddleware(), ListObjectsV2Handler)
	r.PUT("/:bucket/*key", PutObject)

	if err := r.Run(addr); err != nil {
		fmt.Printf("\033[31m[ERROR] \033[0mError starting the server: %v\n", err)
	}
}
