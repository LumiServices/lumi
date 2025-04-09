package cmd

import (
	"bufio"
	"fmt"
	"log"
	"os"

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

			db, err := core.ConnectDB()
			if err != nil {
				log.Fatalf("Failed to connect to database: %v", err)
			}
			defer db.Close()

			tx, err := db.Begin()
			if err != nil {
				log.Fatalf("Failed to begin transaction: %v", err)
			}

			_, err = tx.Exec(`
			CREATE TABLE IF NOT EXISTS server (
				accesskey VARCHAR(255) NOT NULL,
				secretkey VARCHAR(255) NOT NULL
			);`)
			if err != nil {
				tx.Rollback()
				log.Fatalf("Failed to create table: %v", err)
			}

			records, err := tx.Query("SELECT accesskey, secretkey FROM server")
			if err != nil {
				tx.Rollback()
				log.Fatalf("Failed to query database: %v", err)
			}
			defer records.Close()

			if records.Next() {
				var existingAccessKey, existingSecretKey string
				if err := records.Scan(&existingAccessKey, &existingSecretKey); err != nil {
					tx.Rollback()
					log.Fatalf("Failed to scan existing keys: %v", err)
				}
				if existingAccessKey != "" && existingSecretKey != "" {
					fmt.Print("You already have an access key and secret key, would you like to generate new keys? (y/n): ")
					reader := bufio.NewReader(os.Stdin)
					response, _ := reader.ReadString('\n')
					if response != "y\n" && response != "Y\n" {
						tx.Commit()
						return
					}
				}
			}

			_, err = tx.Exec("INSERT INTO server (accesskey, secretkey) VALUES (?, ?)", accessKey, secretKey)
			if err != nil {
				tx.Rollback()
				log.Fatalf("Failed to insert credentials into database: %v", err)
			}

			err = tx.Commit()
			if err != nil {
				log.Fatalf("Failed to commit transaction: %v", err)
			}
			fmt.Print("\033[H\033[2J")
			fmt.Printf("Access Key: %s\n", accessKey)
			fmt.Printf("Secret Key: %s\n", secretKey)
			fmt.Println("Write these down somewhere safe, you won't be able to see them again.")
		},
	}

	return cmd
}
