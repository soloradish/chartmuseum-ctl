# ChartMuseum ctl

## Usage
```
>> cm-ctl help

ChartMuseum command line interface

USAGE:
    cm-cli --url <URL> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -u, --url <URL>    ChartMuseum URL with scheme and port, eg: http://127.0.0.1:8080. [env:
                       CHARTMUSEUM_URL=https://charts.p.cyberx-ops.com/devops]

SUBCOMMANDS:
    delete      delete given version of chart
    help        Prints this message or the help of the given subcommand(s)
    list        list charts name in chartmuseum
    versions    list all version of chart
    
#########################################
>> cm-cli list --help

USAGE:
    cm-cli --url <URL> list

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
    
#########################################
>> cm-cli versions --help

USAGE:
    cm-cli --url <URL> versions [OPTIONS] <CHART>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --before <DAYS>         return only versions upload before [n] days [default: 0]
    -e, --regex <REGEX_EXPR>    match version with regex expressions [default: .*]

ARGS:
    <CHART>
    
#########################################
>> cm-cli delete --help

USAGE:
    cm-cli --url <URL> delete <CHART> <VERSION>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <CHART>
    <VERSION>
```

## Common use case

### Delete chart versions created before 30 days
```
# config chartmuseum endpoint, eg: http://127.0.0.1:8080
export CHARTMUSEUM_URL=http://127.0.0.1:8080
cm-cli versions example-chart --before 30 | xargs -tL 1 cm-cli delete example-chart
```
### Delete chart versions match regex expression
```
# config chartmuseum endpoint, eg: http://127.0.0.1:8080
export CHARTMUSEUM_URL=http://127.0.0.1:8080
# delete versions contained "SNAPSHOT"
cm-cli versions example-chart --regex "SNAPSHOT" | xargs -tL 1 cm-cli delete example-chart
```
