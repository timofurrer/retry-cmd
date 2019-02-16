# retry

Retry commands on the command line without all the loops you always used!

```bash
# Stop retrying after 10 tries
retry --max 10 -- curl -I https://unstable.site

# Stop retrying after 10 tries and wait 5 seconds between the each try
retry --max 10 --interval 5 -- curl -I https://unstable.site

# Stop retrying if exit code is 1
retry --exitcode 1 -- curl -I https://unstable.site

# Suppress stdout and stderr from the command
retry --quiet -- curl -I https://unstable.site
```

## Installation

```
cargo install retry-cmd
```
