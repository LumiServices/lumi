package core

import (
	"errors"
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
	Credential string
	region     string //set to auto I guess? (I think I might be right)
}

func readS3Chunk() {

}

//can somebody hit me up on discord and tell me what streaming means 🙏🙏🙏
//oh https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html

func seedSignature() {

}
