package cmd

import (
	"fmt"
	"log"

	"github.com/ros-e/lumi/core"
	"github.com/spf13/cobra"
)

func NewGenerateCMD() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "generate",
		Short: "Generate an access key and secret key",
		Run: func(cmd *cobra.Command, args []string) {
			accessKey, secretKey, err := core.GenerateCredentials()
			if err != nil {
				log.Fatalf("Failed to generate credentials: %v", err)
			}
			fmt.Printf("Access Key: %s\n", accessKey)
			fmt.Printf("Secret Key: %s\n", secretKey)
		},
	}

	return cmd
}
