package cmd

import (
	"github.com/spf13/cobra"
)

func NewUpdateCMD() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "update",
		Short: "Update the client",
		Run: func(cmd *cobra.Command, args []string) {
			//check current version on github
			//will add this later
			println("Checking for updates...")
		},
	}
	return cmd
}
