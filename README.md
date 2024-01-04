### Pulse 
#### A work in progress information bot linking Discord and Github, with more to come

- for now uses a Discord webhook for only posting messages
- recieves POST requests (e.g. from github webhooks) to be processed into custom messages, which are then POST'd to Discord
    - so we can format the Github POST content to our hearts content
 
# To come

- [ ] support for https POST reciepts (does work, just need actual certs, and to bundle them in)
- [ ] verify POST's are from github using the webhook secret
- [ ] Release formatting
- [ ] Pre-release formatting
