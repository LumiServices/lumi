package api

import (
	"fmt"

	"github.com/gin-gonic/gin"
	"github.com/ros-e/lumi/core"
)

func Serve(addr string, startbanner bool, debug bool) {
	if debug {
		gin.SetMode(gin.DebugMode)
	} else {
		gin.SetMode(gin.ReleaseMode)
	}
	// fmt.Print("\033[35m" + `

	//  ▄█       ███    █▄    ▄▄▄▄███▄▄▄▄    ▄█
	// ███       ███    ███ ▄██▀▀▀███▀▀▀██▄ ███
	// ███       ███    ███ ███   ███   ███ ███▌
	// ███       ███    ███ ███   ███   ███ ███▌
	// ███       ███    ███ ███   ███   ███ ███▌
	// ███       ███    ███ ███   ███   ███ ███
	// ███▌    ▄ ███    ███ ███   ███   ███ ███
	// █████▄▄██ ████████▀   ▀█   ███   █▀  █▀

	// ` + "\033[0m")
	fmt.Printf("[\033[35m REST API started on \033[0mhttp://%s ]\n", addr)
	//debug
	fmt.Printf("[\033[35m ACCESS KEY \033[0m %s]\n", core.Default_Access_Key)
	fmt.Printf("[\033[35m SECRET KEY \033[0m %s]\n", core.Default_Secret_Key)
	r := gin.Default()
	if err := r.Run(addr); err != nil {
		fmt.Printf("\033[31m[ERROR] \033[0mError starting the server: %v\n", err)
	}
}
