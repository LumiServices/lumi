// this needs a complete rewrite
// rewrote it, I really need to stop pasting from minio & seaweedfs :P
package s3

import (
	"bufio"
	"bytes"
	"crypto/hmac"
	"crypto/sha256"
	"encoding/hex"
	"hash"
	"net/http"
	"regexp"
	"slices"
	"sort"
	"strconv"
	"strings"
	"time"
	"unicode/utf8"
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

func extractSignedHeaders(signedHeaders []string, r *http.Request) (http.Header, APIError) {
	reqHeaders := r.Header
	reqQueries := r.Form
	// find whether "host" is part of list of signed headers.
	// if not return ErrUnsignedHeaders. "host" is mandatory.
	if !slices.Contains(signedHeaders, "host") {
		return nil, APIError{Code: ErrUnsignedHeaders}
	}
	extractedSignedHeaders := make(http.Header)
	for _, header := range signedHeaders {
		// `host` will not be found in the headers, can be found in r.Host.
		// but its always necessary that the list of signed headers containing host in it.
		val, ok := reqHeaders[http.CanonicalHeaderKey(header)]
		if !ok {
			// try to set headers from Query String
			val, ok = reqQueries[header]
		}
		if ok {
			extractedSignedHeaders[http.CanonicalHeaderKey(header)] = val
			continue
		}
		switch header {
		case "expect":
			// Golang http server strips off 'Expect' header, if the
			// client sent this as part of signed headers we need to
			// handle otherwise we would see a signature mismatch.
			// `aws-cli` sets this as part of signed headers.
			//
			// According to
			// http://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.20
			// Expect header is always of form:
			//
			//   Expect       =  "Expect" ":" 1#expectation
			//   expectation  =  "100-continue" | expectation-extension
			//
			// So it safe to assume that '100-continue' is what would
			// be sent, for the time being keep this work around.
			// Adding a *TODO* to remove this later when Golang server
			// doesn't filter out the 'Expect' header.
			extractedSignedHeaders.Set(header, "100-continue")
		case "host":
			// Go http server removes "host" from Request.Header
			extractedSignedHeaders.Set(header, r.Host)
		case "transfer-encoding":
			// Go http server removes "host" from Request.Header
			extractedSignedHeaders[http.CanonicalHeaderKey(header)] = r.TransferEncoding
		case "content-length":
			// Signature-V4 spec excludes Content-Length from signed headers list for signature calculation.
			// But some clients deviate from this rule. Hence we consider Content-Length for signature
			// calculation to be compatible with such clients.
			extractedSignedHeaders.Set(header, strconv.FormatInt(r.ContentLength, 10))
		default:
			return nil, APIError{Code: ErrUnsignedHeaders}
		}
	}
	return extractedSignedHeaders, APIError{Code: ErrNone}
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

func parseCredentialHeader(s string) (credentialHeader, ErrorCode) {
	panic("unimplemented")
}

func getStringToSign(canonicalRequest string, t time.Time, scope string) string {
	stringToSign := signV4Algorithm + "\n" + t.Format(iso8601Format) + "\n"
	stringToSign += scope + "\n"
	canonicalRequestBytes := sha256.Sum256([]byte(canonicalRequest))
	stringToSign += hex.EncodeToString(canonicalRequestBytes[:])
	return stringToSign
}

var reservedObjectNames = regexp.MustCompile("^[a-zA-Z0-9-_.~/]+$")

func encodePath(pathName string) string {
	if reservedObjectNames.MatchString(pathName) {
		return pathName
	}
	var encodedPathname string
	for _, s := range pathName {
		if 'A' <= s && s <= 'Z' || 'a' <= s && s <= 'z' || '0' <= s && s <= '9' { // §2.3 Unreserved characters (mark)
			encodedPathname = encodedPathname + string(s)
			continue
		}
		switch s {
		case '-', '_', '.', '~', '/': // §2.3 Unreserved characters (mark)
			encodedPathname = encodedPathname + string(s)
			continue
		default:
			len := utf8.RuneLen(s)
			if len < 0 {
				// if utf8 cannot convert return the same string as is
				return pathName
			}
			u := make([]byte, len)
			utf8.EncodeRune(u, s)
			for _, r := range u {
				hex := hex.EncodeToString([]byte{r})
				encodedPathname = encodedPathname + "%" + strings.ToUpper(hex)
			}
		}
	}
	return encodedPathname
}
func signV4TrimAll(input string) string {
	// Compress adjacent spaces (a space is determined by
	// unicode.IsSpace() internally here) to one space and return
	return strings.Join(strings.Fields(input), " ")
}
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
			buf.WriteString(signV4TrimAll(v))
		}
		buf.WriteByte('\n')
	}
	return buf.String()
}
func getSignedHeaders(signedHeaders http.Header) string {
	var headers []string
	for k := range signedHeaders {
		headers = append(headers, strings.ToLower(k))
	}
	sort.Strings(headers)
	return strings.Join(headers, ";")
}
func getCanonicalRequest(extractedSignedHeaders http.Header, payload, queryStr, urlPath, method string) string {
	rawQuery := strings.ReplaceAll(queryStr, "+", "%20")
	encodedPath := encodePath(urlPath)
	canonicalRequest := strings.Join([]string{
		method,
		encodedPath,
		rawQuery,
		getCanonicalHeaders(extractedSignedHeaders),
		getSignedHeaders(extractedSignedHeaders),
		payload,
	}, "\n")
	return canonicalRequest
}

func calculateSeedSignature(r *http.Request, trailers bool) (cred Credentials, signature string, region string, date time.Time) {
	req := *r
	v4Auth := req.Header.Get("Authorization")
	if v4Auth == "" {
		return cred, "", "", time.Time{}
	}

	signV4Values, errCode := parseSignV4(v4Auth, region, serviceS3)
	if errCode != ErrNone {
		return cred, "", "", time.Time{}
	}
	extractedSignedHeaders, errCode := extractSignedHeaders(signV4Values.SignedHeaders, r)
	if errCode != ErrNone {
		return cred, "", "", time.Time{}, errCode
	}
	canonicalRequest := getCanonicalRequest(extractedSignedHeaders, payload, queryStr, req.URL.Path, req.Method)
	stringToSign := getStringToSign()
	signingKey := getSigningKey(cred.SecretKey, signV4Values.Credential.scope.date, region, serviceS3)
	newSignature := getSignature(signingKey, stringToSign)

	return cred, newSignature, region, date, ErrNone
}

func parseSignV4(v4Auth string, region string, stype serviceType) (sv signValues, aec ErrorCode) {
	// Replace all spaced strings, some clients can send spaced
	// parameters and some won't. So we pro-actively remove any spaces
	// to make parsing easier.
	v4Auth = strings.Replace(v4Auth, " ", "", -1)
	if v4Auth == "" {
		return sv, ErrAuthHeaderEmpty
	}

	// Verify if the header algorithm is supported or not.
	if !strings.HasPrefix(v4Auth, signV4Algorithm) {
		return sv, ErrSignatureVersionNotSupported
	}

	// Strip off the Algorithm prefix.
	v4Auth = strings.TrimPrefix(v4Auth, signV4Algorithm)
	authFields := strings.Split(strings.TrimSpace(v4Auth), ",")
	if len(authFields) != 3 {
		return sv, ErrMissingFields
	}

	// Initialize signature version '4' structured header.
	signV4Values := signValues{}

	var err ErrorCode
	// Save credential values.
	signV4Values.Credential, err = parseCredentialHeader(authFields[0])
	if err != ErrNone {
		return sv, err
	}

	// Save signed headers.
	signV4Values.SignedHeaders, err = parseSignedHeader(authFields[1])
	if err != ErrNone {
		return sv, err
	}

	// Save signature.
	signV4Values.Signature, err = parseSignature(authFields[2])
	if err != ErrNone {
		return sv, err
	}

	// Return the structure here.
	return signV4Values, ErrNone
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
