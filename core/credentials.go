// reimplemention from minio
package core

import (
	"crypto/rand"
	"crypto/subtle"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"
)

const (
	minimum_length_accesskey = 3
	maximum_length_accesskey = 20
	minimum_length_secretkey = 8
	maximum_length_secretkey = 40
	alphaNumericTable        = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	alphaNumericTableLength  = len(alphaNumericTable)
)

var (
	Error_Invalid_Access_Key_Length = fmt.Errorf("access key must be minimum %v or more characters long", minimum_length_accesskey)
	Error_Invalid_Secret_Key_Length = fmt.Errorf("secret key must be minimum %v or more characters long", minimum_length_secretkey)
)

func IsAccessKeyValid(accessKey string) bool {
	return len(accessKey) >= minimum_length_accesskey
}

func IsSecretKeyValid(secretKey string) bool {
	return len(secretKey) >= minimum_length_secretkey
}

const (
	Default_Access_Key = "vultures2"       // <- change this lmfao
	Default_Secret_Key = "used2shop@aldis" // <- change this lmfao (x2)
)

type Credentials struct {
	AccessKey    string    `xml:"AccessKeyId" json:"accessKey,omitempty"`
	SecretKey    string    `xml:"SecretAccessKey" json:"secretKey,omitempty"`
	Expiration   time.Time `xml:"Expiration" json:"expiration,omitempty"`
	SessionToken string    `xml:"SessionToken" json:"sessionToken,omitempty"`
	Status       string    `xml:"-" json:"status,omitempty"`
	//I AM NEVER ADDING IAM POLICIES #IMTRIM #TRIM FR 😂✌️
}

func (c *Credentials) String() string {
	var s strings.Builder
	s.WriteString(c.AccessKey)
	s.WriteString(":")
	s.WriteString(c.SecretKey)
	if c.SessionToken != "" {
		s.WriteString("\n")
		s.WriteString(c.SessionToken)
	}
	if !c.Expiration.IsZero() && c.Expiration != timeSentinel {
		s.WriteString("\n")
		s.WriteString(c.Expiration.String())
	}
	return s.String()
}

var timeSentinel = time.Unix(0, 0).UTC()

func (c *Credentials) IsExpired() bool {
	if c.Expiration == timeSentinel {
		return false
	}
	return c.Expiration.Before(time.Now().UTC())
}

func (c *Credentials) IsTemp() bool {
	return c.SessionToken != "" && !c.Expiration.IsZero() && !c.Expiration.Equal(timeSentinel)
}

func (c *Credentials) IsValid() bool {
	if c.Status == "off" {
		return false
	}
	return IsAccessKeyValid(c.AccessKey) && IsSecretKeyValid(c.SecretKey) && !c.IsExpired()
}

func (c Credentials) Equal(cc Credentials) bool {
	if !cc.IsValid() {
		return false
	}
	return (c.AccessKey == cc.AccessKey && subtle.ConstantTimeCompare([]byte(c.SecretKey), []byte(c.SecretKey)) == 1 &&
		subtle.ConstantTimeCompare([]byte(c.SessionToken), []byte(cc.SessionToken)) == 1)
}

var ErrInvalidDuration = errors.New("invalid token expiry")

func ExpToInt64(expI interface{}) (expAt int64, err error) {
	switch exp := expI.(type) {
	case string:
		expAt, err = strconv.ParseInt(exp, 10, 64)
	case float64:
		expAt, err = int64(exp), nil
	case int64:
		expAt, err = exp, nil
	case int:
		expAt, err = int64(exp), nil
	case uint64:
		expAt, err = int64(exp), nil
	case uint:
		expAt, err = int64(exp), nil
	case json.Number:
		expAt, err = exp.Int64()
	case time.Duration:
		expAt, err = time.Now().UTC().Add(exp).Unix(), nil
	case nil:
		expAt, err = 0, nil
	default:
		expAt, err = 0, ErrInvalidDuration
	}
	if expAt < 0 {
		return 0, ErrInvalidDuration
	}
	return expAt, err
}

func GenerateCredentials() (accessKey, secretKey string, err error) {
	accessKey, err = GenerateACSKey(maximum_length_accesskey, rand.Reader)
	if err != nil {
		return "", "", err
	}
	secretKey, err = GenerateSKey(maximum_length_secretkey, rand.Reader)
	if err != nil {
		return "", "", err
	}
	return accessKey, secretKey, nil
}

func GetNewCredentialsWithMetadata(m map[string]interface{}, tokenSecret string) (Credentials, error) {
	accessKey, secretKey, err := GenerateCredentials()
	if err != nil {
		return Credentials{}, err
	}
	return CreateNewCredentialsWithMetadata(accessKey, secretKey, m, tokenSecret)
}

func GetNewCredentials() (cred Credentials, err error) {
	return GetNewCredentialsWithMetadata(map[string]interface{}{}, "")
}

func CreateNewCredentialsWithMetadata(accessKey, secretKey string, m map[string]interface{}, tokenSecret string) (cred Credentials, err error) {
	if len(accessKey) < minimum_length_accesskey || len(accessKey) > maximum_length_accesskey {
		return Credentials{}, Error_Invalid_Access_Key_Length
	}

	if len(secretKey) < minimum_length_secretkey || len(secretKey) > maximum_length_secretkey {
		return Credentials{}, Error_Invalid_Secret_Key_Length
	}

	cred.AccessKey = accessKey
	cred.SecretKey = secretKey

	if tokenSecret == "" {
		cred.Expiration = timeSentinel
		return cred, nil
	}

	expiry, err := ExpToInt64(m["exp"])
	if err != nil {
		return cred, err
	}
	cred.Expiration = time.Unix(expiry, 0).UTC()

	cred.SessionToken, err = CreateSessionToken(cred.AccessKey, m, tokenSecret)
	if err != nil {
		return cred, err
	}

	return cred, nil
}

func CreateCredentials(accessKey, secretKey string) (cred Credentials, err error) {
	if !IsAccessKeyValid(accessKey) {
		return cred, Error_Invalid_Access_Key_Length
	}
	if !IsSecretKeyValid(secretKey) {
		return cred, Error_Invalid_Secret_Key_Length
	}
	cred.AccessKey = accessKey
	cred.SecretKey = secretKey
	cred.Expiration = timeSentinel
	return cred, nil
}
