package cmd

import (
	"github.com/ros-e/lumi/api"
	"github.com/spf13/cobra"
)

func NewServeCMD() *cobra.Command {
	var httpaddr string
	var ShowStartBanner bool
	var debug bool
	cmd := &cobra.Command{
		Use:   "serve",
		Short: "Start the server",
		Run: func(cmd *cobra.Command, args []string) {
			if httpaddr == "" {
				httpaddr = "0.0.0.0:2100"
			}
			api.Serve(
				httpaddr,
				ShowStartBanner,
				debug)
		},
	}

	cmd.Flags().StringVar(&httpaddr, "http", "", "TCP address to listen for the HTTP server")
	cmd.Flags().BoolVar(&ShowStartBanner, "ShowStartBanner", true, "Hide the start banner")
	cmd.Flags().BoolVar(&debug, "debug", false, "Enable debug mode")

	return cmd
}
