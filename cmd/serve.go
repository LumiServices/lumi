package cmd

import (
	"github.com/ros-e/lumi/api"
	"github.com/spf13/cobra"
)

func NewServeCMD() *cobra.Command {
	var addr string
	var hideStartBanner bool
	var debug bool
	cmd := &cobra.Command{
		Use:   "serve",
		Short: "Start the server",
		Run: func(cmd *cobra.Command, args []string) {
			if len(args) == 0 {
				if addr == "" {
					addr = "0.0.0.0:80"
				} else {
					if addr == "" {
						addr = "0.0.0.0:8090"
					}
				}
				api.Serve(
					addr,
					hideStartBanner,
					debug,
				)
			}
		},
	}
	cmd.PersistentFlags().StringVar(
		&addr,
		"http",
		"",
		"TCP address to listen for the HTTP server\n(if domain args are specified - default to 0.0.0.0:80, otherwise - default to 127.0.0.1:8090)",
	)
	cmd.PersistentFlags().BoolVar(
		&hideStartBanner,
		"hide-start-banner",
		false,
		"Hide the start banner",
	)
	cmd.PersistentFlags().BoolVar(
		&debug,
		"debug",
		false,
		"Enable debug mode",
	)

	return cmd
}
