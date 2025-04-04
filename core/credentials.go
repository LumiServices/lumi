package core

import (
	"fmt"
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
