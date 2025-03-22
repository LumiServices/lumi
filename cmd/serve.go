package cmd

import (
	"github.com/ros-e/lumi/api"
	"github.com/spf13/cobra"
)

func NewServeCMD() *cobra.Command {
	var addr string
	var ShowStartBanner bool
	var debug bool
	cmd := &cobra.Command{
		Use:   "serve",
		Short: "Start the server",
		Run: func(cmd *cobra.Command, args []string) {
			if addr == "" {
				addr = "0.0.0.0:80"
			}
			api.Serve(addr, ShowStartBanner, debug)
		},
	}

	cmd.Flags().StringVar(&addr, "http", "", "TCP address to listen for the HTTP server")
	cmd.Flags().BoolVar(&ShowStartBanner, "ShowStartBanner", true, "Hide the start banner")
	cmd.Flags().BoolVar(&debug, "debug", false, "Enable debug mode")

	return cmd
}
