// https://examplebucket.s3.amazonaws.com/test.txt
// ?X-Amz-Algorithm=AWS4-HMAC-SHA256
// &X-Amz-Credential=<your-access-key-id>/20130721/us-east-1/s3/aws4_request
// &X-Amz-Date=20130721T201207Z
// &X-Amz-Expires=86400
// &X-Amz-SignedHeaders=host
// &X-Amz-Signature=<signature-value>
package s3

import (
	"bytes"
	"crypto/sha256"
	"crypto/subtle"
	"encoding/hex"
	"net/http"
	"net/url"
	"sort"
	"strings"
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

const (
	signV4Algorithm = "AWS4-HMAC-SHA256"
	iso8601Format   = "20060102T150405Z"
	yyyymmdd        = "20060102"
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

// getCanonicalHeaders generate a list of request headers with their values
func getCanonicalHeaders(signedHeaders http.Header) string {
	var headers []string
	vals := make(http.Header)
	for k, vv := range signedHeaders {
		headers = append(headers, strings.ToLower(k))
		vals[strings.ToLower(k)] = vv
	}
	sort.Strings(headers)

	var buf bytes.Buffer
	for _, k := range headers {
		buf.WriteString(k)
		buf.WriteByte(':')
		for idx, v := range vals[k] {
			if idx > 0 {
				buf.WriteByte(',')
			}
			buf.WriteString(core.SignV4TrimAll(v))
		}
		buf.WriteByte('\n')
	}
	return buf.String()
}

// getSignedHeaders generate a string i.e alphabetically sorted, semicolon-separated list of lowercase request header names
func getSignedHeaders(signedHeaders http.Header) string {
	var headers []string
	for k := range signedHeaders {
		headers = append(headers, strings.ToLower(k))
	}
	sort.Strings(headers)
	return strings.Join(headers, ";")
}

// <HTTPMethod>\n
// <CanonicalURI>\n
// <CanonicalQueryString>\n
// <CanonicalHeaders>\n
// <SignedHeaders>\n
// <HashedPayload>
func getCanonicalRequest(extractedSignedHeaders http.Header, payload, queryStr, urlPath, method string) string {
	query := strings.ReplaceAll(queryStr, "+", "%20")
	canonicalRequest := strings.Join([]string{
		method,
		core.EncodePath(urlPath),
		query,
		getCanonicalHeaders(extractedSignedHeaders),
		getSignedHeaders(extractedSignedHeaders),
		payload,
	}, "\n")
	return canonicalRequest
}

// algorithm(dont need this) AWS4-HMAC-SHA256
// date 20130524T000000Z
// scope 20130524/us-east-1/s3/aws4_request
// what the final hash should be 7344ae5b7ee6c3e7e6b0fe0640412a37625d1fbfff95c48bbb2dc43964946972
func getStringToSign(CanonicalRequest string, date time.Time, region string, scope string) string {
	stringToSign := signV4Algorithm + "\n" + date.Format(iso8601Format) + "\n"
	stringToSign = stringToSign + scope + "\n"
	CanonicalRequestBytes := sha256.Sum256([]byte(CanonicalRequest))
	stringToSign = stringToSign + hex.EncodeToString(CanonicalRequestBytes[:])
	return stringToSign
}

func getSigningKey(secretkey string, date time.Time, region string, service serviceType) []byte {
	d := core.SumHMAC([]byte("AWS4"+secretkey), []byte(date.Format(yyyymmdd)))
	regbytes := core.SumHMAC(d, []byte(region))
	stype := core.SumHMAC(regbytes, []byte(service))
	signingKey := core.SumHMAC(stype, []byte("aws4_request"))
	return signingKey
}

func getSignature(signingKey []byte, stringToSign string) string {
	return hex.EncodeToString(core.SumHMAC(signingKey, []byte(stringToSign)))
}

// compareSignatureV4 returns true if and only if both signatures
// are equal. The signatures are expected to be HEX encoded strings
// according to the AWS S3 signature V4 spec.
func compareSignatureV4(sig1, sig2 string) bool {
	// The CTC using []byte(str) works because the hex encoding
	// is unique for a sequence of bytes. See also compareSignatureV2.
	return subtle.ConstantTimeCompare([]byte(sig1), []byte(sig2)) == 1
}

func doesPreSignedSignatureMatch(payload string, r *http.Request, region string) {
	req := *r

}

type preSignValues struct {
	signValues
	Date    time.Time
	Expires time.Duration
}

func doesV4PresignParamsExist(query url.Values) ErrorCode {
	v4PresignQueryParams := []string{"X-Amz-Algorithm", "X-Amz-Credential", "X-Amz-Signature", "X-Amz-Date", "X-Amz-SignedHeaders", "X-Amz-Expires"}
	for _, v4PresignQueryParam := range v4PresignQueryParams {
		if _, ok := query[v4PresignQueryParam]; !ok {
			return ErrInvalidQueryParams
		}
	}
	return ErrNone
}

func parsePsignV4(query url.Values) (psv preSignValues, aec ErrorCode) {
	var err ErrorCode
	aec = doesV4PresignParamsExist(query)
	if aec != ErrNone {
		return psv, err
	}
	if query.Get("X-Amz-Algorithm") != signV4Algorithm {
		return psv, ErrInvalidQuerySignatureAlgo
	}
	preSignV4Values := preSignValues{}
	preSignV4Values.Credential, err = parseCredentialHeader("Credential=" + query.Get("X-Amz-Credential"))
	if err != ErrNone {
		return psv, err
	}
	return preSignV4Values, ErrNone
}

func parseCredentialHeader(credStr string) (credentialHeader, ErrorCode) {
	return credentialHeader{}, ErrNone //placeholder
}
