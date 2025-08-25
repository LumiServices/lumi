package core

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

var Version = "beta-0.0.1"

func GetLatestGitHubRelease() (string, error) {
	url := "https://api.github.com/repos/ros-e/lumi/releases/latest"
	resp, err := http.Get(url)
	if err != nil {
		return "", err
	}
	defer resp.Body.Close()
	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("GitHub API error: %s", string(body))
	}
	var release struct {
		TagName string `json:"tag_name"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&release); err != nil {
		return "", err
	}
	return release.TagName, nil
}
