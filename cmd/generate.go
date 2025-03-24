package cmd

import (
	"log"

	"github.com/ros-e/lumi/core"
	"github.com/spf13/cobra"
)

func NewGenerateCMD() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "generate",
		Short: "Start the server",
		Run: func(cmd *cobra.Command, args []string) {
			db, err := core.ConnectDB()
			if err != nil {
				log.Fatalf("Failed to connect to DB: %v", err)
			}
			defer db.Close()
		},
	}

	return cmd
}
