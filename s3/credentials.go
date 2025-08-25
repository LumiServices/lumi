package s3

import (
	"crypto/rand"
	"encoding/base64"
	"errors"
	"fmt"
	"io"
	"net/http"
	"strings"
)

const (
	MinLeg_AccessKey  = 6
	MaxLeg_AccessKey  = 20
	MinLeg_SecretKey  = 6
	MaxLeg_SecretKey  = 40
	alphaNumericTable = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	// specialCharTable  = "!@#$%^&*()-_+={}[]|:;<>,.?/"
	reservedChars = "=,"
)

var (
	ErrInvalidAccessKeyLength   = fmt.Errorf("access key length should be between %d and %d", MinLeg_AccessKey, MaxLeg_AccessKey)
	ErrInvalidSecretKeyLength   = fmt.Errorf("secret key length should be between %d and %d", MinLeg_SecretKey, MaxLeg_SecretKey)
	ErrNoAccessKeyWithSecretKey = fmt.Errorf("access key must be specified if secret key is specified")
	ErrNoSecretKeyWithAccessKey = fmt.Errorf("secret key must be specified if access key is specified")
	ErrContainsReservedChars    = fmt.Errorf("access key contains one of reserved characters '=' or ','")
)

// Default credentials (if youre too stupid to setup your own)
const (
	DefaultAccessKey = "lumiserver"
	DefaultSecretKey = "lumiserver"
)

func ContainsReservedChars(s string) bool {
	return strings.ContainsAny(s, reservedChars)
}

type Credentials struct {
	AccessKey string
	SecretKey string
}

func GenerateAccessKey(length int, random io.Reader) (string, error) {
	if random == nil {
		random = rand.Reader
	}
	if length <= 0 {
		length = MaxLeg_AccessKey
	}
	if length < MinLeg_AccessKey {
		return "", errors.New("auth: access key length is too short")
	}

	key := make([]byte, length)
	if _, err := io.ReadFull(random, key); err != nil {
		return "", err
	}
	for i := range key {
		key[i] = alphaNumericTable[key[i]%byte(len(alphaNumericTable))]
	}
	return string(key), nil
}

func GenerateSecretKey(length int, random io.Reader) (string, error) {
	if random == nil {
		random = rand.Reader
	}
	if length <= 0 {
		length = MaxLeg_SecretKey
	}
	if length < MinLeg_SecretKey {
		return "", errors.New("auth: secret key length is too short")
	}

	key := make([]byte, base64.RawStdEncoding.DecodedLen(length))
	if _, err := io.ReadFull(random, key); err != nil {
		return "", err
	}

	s := base64.RawStdEncoding.EncodeToString(key)
	return strings.ReplaceAll(s, "/", "+"), nil
}

func ValidateAccessKey(r *http.Request, accessKey string) (Credentials, bool, ErrorCode) {
	cred, isHeader, err := getRequestKey(r)
	if err != ErrNone {
		return Credentials{}, false, err
	}

	if cred.AccessKey == "" {
		return Credentials{}, false, ErrMissingCredTag
	}
	if accessKey != "" && cred.AccessKey != accessKey {
		return Credentials{}, false, ErrInvalidAccessKeyID
	}
	return cred, isHeader, ErrNone

}
