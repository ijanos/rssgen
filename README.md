# rssgen - RSS generator

Generate RSS feeds for things that doesn't have one. rssgen uploads to S3
compatible object storage, Cloudflare's R2 in particular.

You can run rssgen behind a firwall or NAT, you don't need to forward ports.
The outside world only interacts with R2. If rssgen stops the feed will still
be avaialbe, but of course it won't update.

You can use a cron job, a systemd timer a shell script with sleep or you can
run it just once in a while.



# Example script

```
#!/usr/bin/env bash

export RUST_LOG=info
export RSSGEN_S3_ENDPOINT="bucket.r2.cloudflarestorage.com"
export RSSGEN_ACCESS_KEY_ID=accesskey
export RSSGEN_SECRET_ACCESS_KEY=secretkey

while true; do
    ./rssgen
    sleep 1800
done
```
