package api

import (
	"github.com/gin-gonic/gin"
	"github.com/ros-e/lumi/s3"
)

func WriteErrorResponse(c *gin.Context, errorCode s3.ErrorCode, resource string) {
	apiErr := s3.GetAPIError(errorCode)
	errorResp := s3.RESTErrorResponse{
		Code:       apiErr.Code,
		Message:    apiErr.Description,
		Resource:   resource,
		RequestID:  c.GetHeader("X-Amz-Request-Id"),
		BucketName: c.Param("bucket"),
		StatusCode: apiErr.HTTPStatusCode,
	}

	c.XML(apiErr.HTTPStatusCode, errorResp)
}
