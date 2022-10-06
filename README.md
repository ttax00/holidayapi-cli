# Holiday CLI
[![GitHub license](https://img.shields.io/github/license/TechTheAwesome/holidayapi-cli?style=for-the-badge)](https://github.com/TechTheAwesome/holidayapi-cli/blob/main/LICENSE)
[![Test Status](https://img.shields.io/github/workflow/status/techtheawesome/holidayapi-cli/Rust?style=for-the-badge)](https://github.com/TechTheAwesome/holidayapi-cli/actions)
[![wakatime](https://wakatime.com/badge/user/4312729e-bc28-4bc0-9074-161a64a7ad20/project/90c767c8-e677-49e8-853a-7bfc617649c8.svg?style=for-the-badge)](https://wakatime.com/badge/user/4312729e-bc28-4bc0-9074-161a64a7ad20/project/90c767c8-e677-49e8-853a-7bfc617649c8)

Unofficial command line interface for [Holiday API](https://holidayapi.com/) written in [Rust](https://github.com/rust-lang/).

Based on [holidayapi-rust](https://github.com/rust-lang/); Depends on `clap` and `confy` for arguments and configuration parsing. 

## Getting started

## Examples
Setting up key
```console
Usage: holidayapi-cli key [OPTIONS]

Options:
  -k, --key <key>  
  -h, --help       Print help information
```
Holidays endpoint
```console
Usage: hapi holiday [OPTIONS] --country <COUNTRY> --year <YEAR>

Options:
  -k, --key <KEY>            A temporary API key, won't be save to config, and overrides config for this request
  -c, --countries <COUNTRY>    For countries, ISO 3166-1 alpha-2 or ISO 3166-1 alpha-3 format
  -y, --years <YEAR>          year in ISO 8601 format
  -m, --month <1-12>         1 or 2 digit month (1-12)
  -d, --day <1-31>           1 or 2 digit day (1-31 depending on the month)
      --public               Return only public holidays
  -s, --subdivisions         Return state / province holidays alongside countrywide holidays
      --search <SEARCH>      Search holidays by name. Minimum 5 characters
  -l, --language <LANGUAGE>  ISO 639-1 format (with exceptions)
      --previous             Return the first day of holidays that occur before the specific date
      --upcoming             Return the first day of holidays that occur after the specific date
  -f, --format <FORMAT>      Response format (csv, json, php, tsv, yaml and xml) [default: json]
  -p, --pretty               Prettifies results to be more human-readable
  -h, --help                 Print help information (use `--help` for more detail)
  ```
  Countries endpoint
  ```console
  Usage: hapi country [OPTIONS]

Options:
  -k, --key <KEY>          A temporary API key, won't be save to config, and overrides config for this request
  -c, --country <COUNTRY>  Return only the country with the specified code
  -s, --search <SEARCH>    Search countries by code and name. Minimum 2 characters
      --public             Return only public holidays
  -f, --format <FORMAT>    Response format (csv, json, php, tsv, yaml and xml). [default: json]
  -p, --pretty             Prettifies results to be more human-readable
  -h, --help               Print help information
  ```