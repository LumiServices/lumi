package core

import (
	"crypto/rand"
	"encoding/base64"
	"errors"
	"io"
	"strings"

	"github.com/golang-jwt/jwt/v5"
)

var Version = "0"

// AccessKey //
func GenerateACSKey(length int, random io.Reader) (string, error) {
	if random == nil {
		random = rand.Reader
	}
	if length < 0 {
		length = maximum_length_accesskey
	}
	if length < minimum_length_accesskey {
		return "", errors.New("auth: access key length is too short")
	}
	key := make([]byte, length)
	if _, err := io.ReadFull(random, key); err != nil {
		return "", err
	}
	for i := range key {
		key[i] = alphaNumericTable[key[i]%byte(alphaNumericTableLength)]
	}
	return string(key), nil
}

// SecretKey //
func GenerateSKey(length int, random io.Reader) (string, error) {
	if random == nil {
		random = rand.Reader
	}
	if length <= 0 {
		length = maximum_length_secretkey
	}
	if length < minimum_length_secretkey {
		return "", errors.New("auth: secret key length is too short")
	}

	key := make([]byte, base64.RawStdEncoding.DecodedLen(length))
	if _, err := io.ReadFull(random, key); err != nil {
		return "", err
	}

	s := base64.RawStdEncoding.EncodeToString(key)
	return strings.ReplaceAll(s, "/", "+"), nil
}

func CreateSessionToken(accessKey string, m map[string]interface{}, tokenSecret string) (string, error) {
	m["accessKey"] = accessKey
	jwt := jwt.NewWithClaims(jwt.SigningMethodHS512, jwt.MapClaims(m))
	return jwt.SignedString([]byte(tokenSecret))
}

func ExtractClaimstoken(token, secretKey string) (*jwt.MapClaims, error) {
	if token == "" || secretKey == "" {
		return nil, errors.New("invalid argument")
	}

	claims := jwt.MapClaims{}

	stsTokenCallback := func(token *jwt.Token) (interface{}, error) {
		return []byte(secretKey), nil
	}

	_, err := jwt.ParseWithClaims(token, claims, stsTokenCallback)
	if err != nil {
		return nil, err
	}

	return &claims, nil
}

// Retreives the Access Key from database
func GetAccessKey() (string, error) {
	db, err := ConnectDB()
	if err != nil {
		return "", err
	}
	defer db.Close()
	var key string
	row := db.QueryRow("SELECT accesskey FROM server WHERE id = ?")
	if err := row.Scan(&key); err != nil {
		return "", err
	}
	return key, nil
}
