package lumi

import (
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/ros-e/lumi/cmd"
	"github.com/ros-e/lumi/core"
	"github.com/spf13/cobra"
)

type App struct {
	Version string
	cmd     *cobra.Command
	Config  Config
}

type Config struct {
	Port    string
	DataDir string
}

func InspectRuntime() (baseDir string, withGoRun bool) {
	if strings.HasPrefix(os.Args[0], os.TempDir()) {
		withGoRun = true
		baseDir, _ = os.Getwd()
	} else {
		withGoRun = false
		baseDir = filepath.Dir(os.Args[0])
	}
	return
}

func NewApp() *App {
	config := Config{}
	if config.DataDir == "" {
		_, _ = InspectRuntime()
		config.DataDir = filepath.Join("data")
	}
	if err := os.MkdirAll(config.DataDir, os.ModePerm); err != nil {
		log.Fatalf("failed to create data directory: %v", err)
	}
	// dbPath := filepath.Join("lumi.db")
	// if _, err := os.Stat(dbPath); os.IsNotExist(err) {
	// 	file, err := os.Create(dbPath)
	// 	if err != nil {
	// 		log.Fatalf("failed to create db: %v", err)
	// 	}
	// 	file.Close()
	// }

	executableName := filepath.Base(os.Args[0])
	rootCmd := &cobra.Command{
		Use:     executableName,
		Short:   "lumi CLI",
		Version: core.Version,
	}
	rootCmd.AddCommand(cmd.NewUpdateCMD())
	rootCmd.AddCommand(cmd.NewServeCMD())
	// rootCmd.PersistentFlags().StringVar(
	// 	&config.DataDir,
	// 	"dir",
	// 	config.DataDir,
	// 	"the lumi data directory",
	// )
	return &App{
		cmd: rootCmd,
		Config: Config{
			Port:    "8080",
			DataDir: config.DataDir,
		},
	}
}

func (a *App) Run() {
	if err := a.cmd.Execute(); err != nil {
		log.Fatalf("could not run the app: %v", err)
	}
}
