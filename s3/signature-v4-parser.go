//(I'm too stupid to find the docs again https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html)

package s3

import (
	"net/http"
	"strings"
)

func getRequestKey(r *http.Request) (Credentials, bool, ErrorCode) {
	authHeader := r.Header.Get("Authorization")
	var accessKey string
	var isHeader bool
	if authHeader != "" {
		const sigV4Algo = "AWS4-HMAC-SHA256 "
		authHeader = strings.TrimPrefix(authHeader, sigV4Algo)
		authFields := strings.Split(authHeader, ",")
		for _, field := range authFields {
			field = strings.TrimSpace(field)
			if strings.HasPrefix(field, "Credential=") {
				credStr := strings.TrimPrefix(field, "Credential=")
				parts := strings.Split(credStr, "/")
				if len(parts) < 1 {
					return Credentials{}, false, ErrCredMalformed
				}
				accessKey = parts[0]
				isHeader = true
				break
			}
		}
	}
	if accessKey == "" {
		credQuery := r.URL.Query().Get("X-Amz-Credential")
		if credQuery == "" {
			return Credentials{}, false, ErrMissingCredTag
		}
		parts := strings.Split(credQuery, "/")
		if len(parts) < 1 {
			return Credentials{}, false, ErrCredMalformed
		}
		accessKey = parts[0]
		isHeader = false
	}
	creds, _, err := ValidateAccessKey(r, accessKey)
	if err != ErrNone {
		return Credentials{}, isHeader, err
	}
	return creds, isHeader, ErrNone
}
