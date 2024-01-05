## Pulse 
### A work in progress information bot linking Discord and Github, with more to come

- for now uses a Discord webhook for only posting messages
- recieves POST requests (e.g. from github webhooks) to be processed into custom messages, which are then POST'd to Discord
    - so we can format the Github POST content to our hearts content
 
### Roadmap

- [x] support for https POST receipts
- [x] support for http POST receipts (as a cargo build option)
- [x] verify POST's are from github using the webhook secret
- [ ] Release formatting
- [ ] Pre-release formatting

### Setup

### Example Google Cloud instance (free tier)

### https certificate setup

#### Self signed (useful for localhost testing)

- You can use the bash script ```certs/gen.sh``` to generate a key/cert pair with openssl 

#### Production; from authority

- get a domain (e.g. from squarespace)
- create a custom DNS record, e.g.
    - ```
        your.domain.somwhere    A	1 hour	google.cloud.instance.ip 
    ```
- Use [Let's Encrypts](https://letsencrypt.org/) recommendation of [certbot](https://certbot.eff.org/) it really is very easy
    - You will need to enable http in the cloud instance firewall for provisioning as well as https