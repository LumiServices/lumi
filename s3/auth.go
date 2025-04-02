package s3

// AWS Signature Version '4' constants.
const (
	signV4Algorithm = "AWS4-HMAC-SHA256"
	signV2Algorithm = "AWS"
	iso8601Format   = "20060102T150405Z"
	yyyymmdd        = "20060102"
)

type Credentials struct {
	AccessKey string `xml:"AccessKeyId" json:"accessKey,omitempty" yaml:"accessKey"`
	SecretKey string `xml:"SecretAccessKey" json:"secretKey,omitempty" yaml:"secretKey"`
}
