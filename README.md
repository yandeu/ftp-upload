# FTP Upload

Upload a entire folder via FTP.

```bash
Usage: ftp-upload [OPTIONS] --dir <DIR> --user <USER> --password <PASSWORD> --host <HOST>

Options:
  -d, --dir <DIR>
  -u, --user <USER>
  -p, --password <PASSWORD>
  -H, --host <HOST>
  -s, --silent <SILENT>      [possible values: true, false]
  -h, --help                 Print help information
```

Example:

```bash
# upload the www folder and its content
./ftp-upload --dir 'www' --user 'test@demo.com' -p 'SOME_PASSWORD' -H 'your.website.com:21'
```

```bash
# upload only the content of the www folder
./ftp-upload --dir 'www/*' --user 'test@demo.com' -p 'SOME_PASSWORD' -H 'your.website.com:21'
```

Download:

- https://github.com/yandeu/ftp-upload/releases
