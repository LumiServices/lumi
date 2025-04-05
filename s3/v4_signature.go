package s3

import (
	"bufio"
	"hash"
	"net/http"
	"time"

	"github.com/ros-e/lumi/core"
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

type credentialHeader struct {
	accessKey string
	scope     struct {
		date    time.Time
		region  string
		service string
		request string
	}
}

type signValues struct {
	Credential    credentialHeader
	SignedHeaders []string
	Signature     string
}

func (s *s3ChunkedReader) getChunkSignature() {

}

// most of this is stolen from https://github.com/minio/minio/blob/b67f0cf72160263bba62664c8e9433132ebdddf0/cmd/streaming-signature-v4.go#L226
type s3ChunkedReader struct {
	SecretKey         string
	cred              *core.Credentials
	reader            *bufio.Reader
	seedSignature     string
	seedDate          time.Time
	region            string
	trailers          http.Header
	chunkSHA256Writer hash.Hash
	buffer            []byte
}
