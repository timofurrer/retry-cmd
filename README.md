# retry

Retry commands on the command line without all the loops you always used!

```bash
retry --max 10 -- curl -I https://unstable.site

retry --max 10 --interval 5 -- curl -I https://unstable.site
```

## Installation

```
cargo install retry-cmd
```
