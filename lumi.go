package lumi

import (
	"log"

	"github.com/ros-e/lumi/cmd"
	"github.com/spf13/cobra"
)

type App struct {
	Version string
	cmd     *cobra.Command
}

const Version = "Alpha"

type Config struct {
	Port            string
	DataDir         string
	HideStartBanner bool
}

func NewApp() *App {
	rootCmd := &cobra.Command{
		Use:     "lumi",
		Version: Version,
		Short:   "open-source object storage service",
	}
	rootCmd.AddCommand((cmd.NewUpdateCMD()))
	rootCmd.AddCommand(cmd.NewServeCMD())

	return &App{
		cmd: rootCmd,
	}
}

func (a *App) Run() {
	if err := a.cmd.Execute(); err != nil {
		log.Fatalf("could not run the app: %v", err)
	}
}
