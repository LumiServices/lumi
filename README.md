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

## Star History

<a href="https://www.star-history.com/#ros-e/lumi&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=ros-e/lumi&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=ros-e/lumi&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=ros-e/lumi&type=Date" />
 </picture>
</a>