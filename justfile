build:
    mkdir -p build
    GOOS=darwin GOARCH=arm64 go build -o build/lumi-mac-arm64 bin/main.go
    GOOS=windows GOARCH=amd64 go build -o build/lumi-windows-amd64.exe bin/main.go
    GOOS=linux GOARCH=amd64 go build -o build/lumi-linux-amd64 bin/main.go
