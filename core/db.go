package core

import (
	"database/sql"
	"fmt"

	_ "github.com/mattn/go-sqlite3"
)

func ConnectDB() (*sql.DB, error) {
	db, err := sql.Open("sqlite3", "lumi.db")
	if err != nil {
		return nil, err
	}
	if err = db.Ping(); err != nil {
		db.Close()
		return nil, err
	}

	fmt.Println("Connected to SQLite database successfully")
	return db, nil
}
