package cmd

import "github.com/spf13/cobra"

func NewServeCMD() *cobra.Command {
	return &cobra.Command{
		Use:   "serve",
		Short: "Starts the web server",
		Run: func(cmd *cobra.Command, args []string) {
		},
	}
}
