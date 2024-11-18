# calcr

Training platform for conversions 


## Usage

```
Usage: calcr [OPTIONS]

Options:
  -p, --port <PORT>                  [default: 8080]
  -b, --bind-address <BIND_ADDRESS>  [default: 0.0.0.0]
  -h, --help                         Print help
```

## Deploy

This project is intended to be run with Docker (but you may also clone and build via cargo yourself).

```
git clone https://github.com/SpaceMoehre/calcr
sudo docker build calcr -t calcr:latest
sudo docker run --name calcr -p 8080:8080 calcr:latest
```

## Notes

`/templates` and other source files are not required for hosting. Only the binary.