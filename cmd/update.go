package cmd

import (
	"fmt"
	"strings"

	"github.com/ros-e/lumi/core"
	"github.com/spf13/cobra"
)

func NewUpdateCMD() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "update",
		Short: "Update the client",
		Run: func(cmd *cobra.Command, args []string) {
			println("\033[33mChecking for updates...\033[0m")
			//request latest version from github
			latestVersion, err := core.GetLatestGitHubRelease()
			if err != nil {
				fmt.Printf("\033[31mFailed to check for updates: %v\033[0m\n", err)
				return
			}
			if strings.TrimPrefix(latestVersion, "v") != strings.TrimPrefix(core.Version, "v") {
				fmt.Printf("\033[36mNew version available: %s (current: %s)\033[0m\n", latestVersion, core.Version)
			} else {
				fmt.Println("\033[32mYou are running the latest version.\033[0m")
			}
		},
	}
	return cmd
}
