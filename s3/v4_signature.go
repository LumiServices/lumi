package s3

import (
	"bufio"
	"errors"
	"io"
	"strconv"
	"strings"
)

type ChecksumAlgorithm int

// result of a blank sha256 string
const EmptyHash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

// Headers used in AWSv4 sigs
const (
	X_AMZ_ALGORITHM      = "x-amz-algorithm"
	X_AMZ_CREDENTIAL     = "x-amz-credential"
	X_AMZ_DATE           = "x-amz-date"
	X_AMZ_EXPIRES        = "x-amz-expires"
	X_AMZ_SIGNEDHEADERS  = "x-amz-signedheaders"
	X_AMZ_SIGNATURE      = "x-amz-signature"
	X_AMZ_CONTENT_SHA256 = "x-amz-content-sha256"
	X_AMZ_TRAILER        = "x-amz-trailer"
)

const (
	ChecksumAlgorithmNone ChecksumAlgorithm = iota
	ChecksumAlgorithmSHA256
)

func extractChecksumAlgorithm(amzTrailerHeader string) (ChecksumAlgorithm, error) {
	//get the checksum from the header example: x-amz-content-sha256
	switch amzTrailerHeader {
	case "x-amz-checksum-sha256":
		return ChecksumAlgorithmSHA256, nil
	default:
		return ChecksumAlgorithmNone, errors.New("unsupported checksum algorithm '" + amzTrailerHeader + "'")
	}
}

type chunkHeaders struct {
	reader         *bufio.Reader
	Credential     string
	region         string
	chunkSize      uint64
	chunkSignature string
}

func (c *chunkHeaders) readS3Chunk() ([]byte, error) {
	headerline, err := c.reader.ReadString('\n')
	if err != nil {
		return nil, err
	}

	chunkSize, chunkSignature, err := parseS3ChunkExtension([]byte(strings.TrimSpace(headerline)))
	if err != nil {
		return nil, err
	}

	c.chunkSize = chunkSize
	c.chunkSignature = chunkSignature

	if c.chunkSize == 0 {
		return nil, io.EOF
	}

	chunkData := make([]byte, c.chunkSize)
	_, err = io.ReadFull(c.reader, chunkData)
	if err != nil {
		return nil, err
	}

	return chunkData, nil
}

func parseS3ChunkExtension(header []byte) (uint64, string, error) {
	parts := strings.Split(string(header), ";")
	chunkSizeHex := parts[0]
	chunkSize, err := strconv.ParseUint(chunkSizeHex, 16, 64)
	if err != nil {
		return 0, "", err
	}
	var chunkSignature string
	if len(parts) > 1 && strings.HasPrefix(parts[1], "chunk-signature=") {
		chunkSignature = strings.Split(parts[1], "=")[1]
	}
	return chunkSize, chunkSignature, nil
}

//https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html

type Request struct {
	Method         string
	CanonicalURI   string
	CanonicalQuery string
	Headers        map[string]string
	Payload        []byte
}

func GenerateSignatureV4(
	AccessKeyID string,
	SecretAccessKey string,
	Region string,
	Service string,
	req Request,
) (string, error) {
	canonicalRequest := createCanonicalRequest(req)
	date := req.Headers[X_AMZ_DATE]
	stringToSign := stringToSign(canonicalRequest, date, Region, Service)
	signature := calculateSignature(SecretAccessKey, date, Region, Service, stringToSign)
	return signature, nil
}

func calculateSignature(SecretAccessKey, date, Region, Service string, stringToSign string) string {
	return ""
}

func createCanonicalRequest(req Request) string {
	return ""
}

func stringToSign(canonicalRequest,
	date,
	region,
	service string) string {
	return ""
}

func seedSignature() {
	// I think I need to seperate the files because idfk what I'm doin anymore...
	// I know what I'm doing now
}
