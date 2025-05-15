package s3

import (
	"net/http"
	"os"
	"strings"
)

// VERY VERY VERY VERYYYY BASIC
func AuthenticateRequest(r *http.Header) ErrorCode {
	authHead := r.Get("Authorization")
	if authHead == "" {
		return ErrMissingCredTag
	}
	credentialheader := strings.Split(authHead, "Credential=")
	if len(credentialheader) < 2 {
		return ErrCredMalformed
	}
	accessKeyHeader := strings.Split(credentialheader[1], "/")
	if len(accessKeyHeader) < 1 {
		return ErrCredMalformed
	}
	accessKey := accessKeyHeader[0]

	expectedKey := os.Getenv("lumi_access_key")
	if expectedKey == "" {
		return ErrAuthNotSetup
	}

	if accessKey != expectedKey {
		return ErrInvalidAccessKeyID
	}
	return ErrNone
}
