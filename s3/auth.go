package s3

import (
	"database/sql"
	"net/http"

	"github.com/ros-e/lumi/core"
)

func checkAccessKey(req *http.Request, accesskey string) (core.Credentials, bool, ErrorCode) {
	// Placeholder until IAM is implemented
	db, err := core.ConnectDB()
	if err != nil {
		return core.Credentials{}, false, ErrNotImplemented
	}
	defer db.Close()

	var key string
	row := db.QueryRow("SELECT accesskey FROM server WHERE accesskey = ?", accesskey)
	if err := row.Scan(&key); err != nil {
		if err == sql.ErrNoRows {
			return core.Credentials{}, false, ErrInvalidAccessKeyID
		}
		return core.Credentials{}, false, ErrNotImplemented
	}
	creds := core.Credentials{
		AccessKey: key,
	}
	return creds, true, ErrNone
}
