# ComBot

A command line utility to parse server access logs, detect bots, and output a list of detected bots in the specified format.

## Supported Input Formats

- [x] NGINX
- [ ] Apache

## Supported Output Formats

- [x] CSV
- [x] [AbuseIPDB](https://abuseipdb.com) CSV (don't forget to dedpulicate this list before submitting)
- [ ] JSON

## Trigger Lists

This tool supports specifying "trigger lists" or using the built in lists.

List files should be specified in the following format:

```
botname1|trigger string one
botname2|trigger string two
```

The pipe `|` character is used to separate the trigger name from the trigger string, each line is a trigger.

Two lists are spported, these are URI lists and User Agent lists. Trigger strings work best when they are a substring of the respective part of the log entry. For example:

For a URI trigger use `phpunit` not `/test/phpunit/submit.php`.

For a User Agent trigger use `zgrab` not `Mozilla/5.0 zgrab/0.x`.

URIs take precedence over User Agents as they tend to be more specific.

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
    <input>     the input 
    file to use.
    <output>    the output file to use.
```
