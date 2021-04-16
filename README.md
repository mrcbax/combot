# ComBot

A command line utility to parse server access logs, detect bots, and output a list of detected bots in the specified format.

## Supported input formats:

- [x] NGINX
- [ ] Apache

## Supported output formats:

- [x] CSV
- [x] [AbuseIPDB](https://abuseipdb.com) CSV
- [ ] JSON

## Help

```
combot 0.1.0	GNU-GPL-3.0
Chad Baxter
A utility to parse server access logs and detect bots based on URI paths and User Agents.

USAGE:
    combot [OPTIONS] <input> <output> --input_format <input_format> --output_format <output_format>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input_format <input_format>      Select the input format: nginx
    -f, --output_format <output_format>    Select the output format: csv, abuseipdb-csv
    -a, --ua_list <ua_list>                Specify a path to a list of User Agent pieces to trigger on.
    -u, --uri_list <uri_list>              Specify a path to a list of URI pieces to trigger on.

ARGS:
    <input>     the input file to use.
    <output>    the output file to use.
```
