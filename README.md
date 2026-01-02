# LUMI

## Downloads
- **Windows**: [Download](https://github.com/ros-e/lumi/releases)
- **macOS**: [Download](https://github.com/ros-e/lumi/releases)
- **Linux**: [Download](https://github.com/ros-e/lumi/releases)

## Build from source
```sh
git clone https://github.com/ros-e/lumi.git
cd lumi && 
```

## Setup
lumi requires authentication credentials to operate. Generate them using
```sh
lumi generate-credentials
```
### add to env
**For bash/zsh:**
```sh
echo 'export lumi_access_key="your-access-key"' >> ~/.bashrc
echo 'export lumi_secret_key="your-secret-key"' >> ~/.bashrc
source ~/.bashrc
```

**For fish:**
```sh
set -Ux lumi_access_key "your-access-key"
set -Ux lumi_secret_key "your-secret-key"
```

## Usage
Once credentials are set, start the server:
```sh
lumi serve
```


## Star History

<a href="https://www.star-history.com/#ros-e/lumi&Date">
 <picture>
   <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=ros-e/lumi&type=Date&theme=dark" />
   <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=ros-e/lumi&type=Date" />
   <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=ros-e/lumi&type=Date" />
 </picture>
</a>
