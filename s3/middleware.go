package s3

import (
	"github.com/gin-gonic/gin"
)

func S3AuthMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		errCode := AuthenticateRequest(&c.Request.Header)
		if errCode != ErrNone {
			apiErr := GetAPIError(errCode)
			errResp := RESTErrorResponse{
				Code:       apiErr.Code,
				Message:    apiErr.Description,
				StatusCode: apiErr.HTTPStatusCode,
				Resource:   c.Request.URL.Path,
				RequestID:  c.GetHeader("X-Amz-Request-Id"), // Include request ID if available
			}
			c.XML(apiErr.HTTPStatusCode, errResp)
			c.Abort()
			return
		}
		c.Next()
	}
}
