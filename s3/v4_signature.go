// this needs a complete rewrite
package s3

import (
	"bufio"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"hash"
	"net/http"
	"strings"
	"time"
)

const (
	emptySHA256                   = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
	streamingContentSHA256        = "STREAMING-AWS4-HMAC-SHA256-PAYLOAD"
	streamingContentSHA256Trailer = "STREAMING-AWS4-HMAC-SHA256-PAYLOAD-TRAILER"
	signV4ChunkedAlgorithm        = "AWS4-HMAC-SHA256-PAYLOAD"
	signV4ChunkedAlgorithmTrailer = "AWS4-HMAC-SHA256-TRAILER"
	streamingContentEncoding      = "aws-chunked"
	awsTrailerHeader              = "X-Amz-Trailer"
	trailerKVSeparator            = ":"
)

type serviceType string

const SlashSeparator = "/"
const serviceS3 serviceType = "s3"

// AWS S3 authentication headers that should be skipped when signing the request
// https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html
var awsS3AuthHeaders = map[string]struct{}{
	"x-amz-content-sha256": {},
	"x-amz-security-token": {},
	"x-amz-algorithm":      {},
	"x-amz-date":           {},
	"x-amz-expires":        {},
	"x-amz-signedheaders":  {},
	"x-amz-credential":     {},
	"x-amz-signature":      {},
}

func (s *s3ChunkedReader) getChunkSignature() string {
	hashedChunk := hex.EncodeToString(s.chunkSHA256Writer.Sum(nil))
	alg := signV4Algorithm + "\n"
	stringToSign := alg +
		s.seedDate.Format(iso8601Format) + "\n" +
		getScope(s.seedDate, s.region) + "\n" +
		s.seedSignature + "\n" +
		emptySHA256 + "\n" +
		hashedChunk

	signingKey := getSigningKey(s.SecretKey, s.seedDate, s.region, serviceS3)
	newSignature := getSignature(signingKey, stringToSign)

	return newSignature
}

func calculateSeedSignature() {

}

func getSigningKey(secretKey string, t time.Time, region string, stype serviceType) []byte {
	date := sumHMAC([]byte("AWS4"+secretKey), []byte(t.Format(yyyymmdd)))
	regionBytes := sumHMAC(date, []byte(region))
	service := sumHMAC(regionBytes, []byte(stype))
	signingKey := sumHMAC(service, []byte("aws4_request"))
	return signingKey
}

func getSignature(signingKey []byte, stringToSign string) string {
	return hex.EncodeToString(sumHMAC(signingKey, []byte(stringToSign)))
}

func getScope(t time.Time, region string) string {
	scope := strings.Join([]string{
		t.Format(yyyymmdd),
		region,
		string(serviceS3),
		"aws4_request",
	}, SlashSeparator)
	return scope
}

func sumHMAC(key []byte, data []byte) []byte {
	hash := hmac.New(sha256.New, key)
	hash.Write(data)
	return hash.Sum(nil)
}

// most of this is stolen from https://github.com/minio/minio/blob/b67f0cf72160263bba62664c8e9433132ebdddf0/cmd/streaming-signature-v4.go#L226
type s3ChunkedReader struct {
	SecretKey         string
	reader            *bufio.Reader
	seedSignature     string
	seedDate          time.Time
	region            string
	trailers          http.Header
	chunkSHA256Writer hash.Hash
	buffer            []byte
}
