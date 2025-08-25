# LUMI (beta)

Download for [**Windows**](https://github.com/ros-e/lumi/releases)
Download for [**macOS**](https://github.com/ros-e/lumi/releases)
Download for [**Linux**](https://github.com/ros-e/lumi/releases)

## Build from source
```sh
git clone https://github.com/ros-e/lumi.git
cd lumi && just build
```

## Setup
Due to standard S3 authentication not being implemented yet, set the `lumi_access_key` environment variable.
```sh
export lumi_access_key="your-access-key"
```
We reccomend using a random hex
```sh
openssl rand -hex 10
```
