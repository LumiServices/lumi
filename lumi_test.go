package lumi

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNewApp(t *testing.T) {
	app := NewApp()
	assert.NotNil(t, app)
	assert.Equal(t, Version, app.cmd.Version)
}

func TestApp_Run(t *testing.T) {
	app := NewApp()
	assert.NotPanics(t, func() {
		go app.Run()
	})
}
