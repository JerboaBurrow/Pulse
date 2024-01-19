## Pulse 
<a href="https://discord.gg/SBeMPQtuB5">
<img width="auto" height="24px" src=https://pulse.jerboa.app:3030/badge alt="Pulse status badge, with a link to the Discord channel. If this text is visible Pulse is sleeping."></img>
</p>
</a>

### A work in progress information bot linking Discord and Github

- **HMAC event authentication** 🔐: filter Github webhook POST events, and authenticate with hotloadable HMAC's per event type
- **Discord message posting with templates** 📟: hotloadable templates per event type
- **Message template interpolation**: automatically fill template from POST JSON bodies, fully hotloadable 
- **Template selection logic** hotloadable template selection depending on values in Github POST body JSON data
- **Hotloadable configuration** 🔥: ssl keys, ports, multiple Discord webhooks endpoints, Github HMAC's, event templates, template selection logic.   

![Screenshot from 2024-01-06 14-21-18](https://github.com/JerboaBurrow/Pulse/assets/84378622/34a58459-a51d-4c80-9d1a-07d6f98551eb)

- recieves POST requests (e.g. from github webhooks) to be processed into custom messages, which are then POST'd to Discord
    - so we can format the Github POST content to our hearts content
 
### Roadmap

- [x] support for HTTPS POST receipts (*HTTP as a cargo build option*)
- [x] verify POST's are from github using the webhook secret
- [x] starred formatting 
- [x] hot loading of formatting templates, webhook endpoints, and github HMACs
- [x] template interpolation, with interpolation data extracted from POST bodies.
- [x] support multiple webhook end-points
- [x] support multiple github webhook origins (i.e. multiple HMACs)
- [ ] suppress messaging on private repos as config option 
- [ ] select templates based upon POST body/header content
- [ ] Release formatting (create, publish)
- [ ] Issue formatting (e.g. when tagged with bug)
- [ ] PR formatting (for new pr's and for successful merges)
- [ ] statistics roundup (set frequency in config)

### Setup

You can host on localhost, or via a remote server with http or https

#### Configuration files

Create a ```event_config.json``` specifying hmac secrets and Discord webhook endpoints, and templates for responses. 

These can include interpolation placeholders, i.e. ```<respository/name>``` searches the JSON body for the path ```repository/name``` and replaces the token with it. [Checkout the payloads here](https://docs.github.com/en/webhooks/webhook-events-and-payloads) **any** JSON path is supported in Pulse if it is in the Github payload.

```json
{
    "github_released":
    {
        "hmac": "this_is_secret_number_one",
        "template": "Newly <action> release for <respository/name>!",
        "end_point": { "addr": "https://discord.com/api/webhooks/a/webhook" }
    },
    "github_starred":
    {
        "hmac": "this_is_secret_number_one",
        "template": "A star was <action> for <repository/name>!",
        "end_point": { "addr": "https://discord.com/api/webhooks/a/webhook" }
    },
    "github_pushed":
    {
        "hmac": "this_is_secret_number_two",
        "template": "New push!",
        "end_point": { "addr": "https://discord.com/api/webhooks/a/webhook" }
    },
    "github_ping":
    {
        "hmac": "this_is_secret_number_three",
        "template": "New ping!",
        "end_point": { "addr": "https://discord.com/api/webhooks/another/webhook" }
    }
}
```

and a ```config.json``` for the server parameters

```json
{
    "port": 3030,
    "stats_endpoint": { "addr": "https://discord.com/api/webhooks/statistics/webhook" },
    "cert_path": "",
    "key_path": ""
}
```

#### Localhost

Just launch the server!

#### Example Google Cloud instance (free tier)

The [gcloud free tier](https://cloud.google.com/free?hl=en) [allows for the following instance running 24/7:](https://cloud.google.com/free/docs/free-cloud-features#compute)

```
    1 non-preemptible e2-micro VM instance per month in one of the following US regions:
        Oregon: us-west1
        Iowa: us-central1
        South Carolina: us-east1
    30 GB-months standard persistent disk
    1 GB of outbound data transfer from North America to all region destinations (excluding China and Australia) per month

```

You may still see costs in the Google cloud console, or savings suggestions. You should not be charged though. I have had it running for years all free.

##### Create it using the CLI...

Using the gloud cli this command should create an instance template for the free tier, which can be used to create instances

```bash
gcloud beta compute instance-templates create free-tier-template-http --project=YOUR_PROJECT --machine-type=e2-micro \\
--network-interface=network=default,network-tier=PREMIUM \\
--instance-template-region=projects/YOUR_PROJECT/regions/us-central1 --maintenance-policy=MIGRATE \\
--provisioning-model=STANDARD --service-account=YOUR_SERVICE_ACCOUNT \\
--scopes=https://www.googleapis.com/auth/devstorage.read_only,https://www.googleapis.com/auth/logging.write,https://www.googleapis.com/auth/monitoring.write,https://www.googleapis.com/auth/servicecontrol,https://www.googleapis.com/auth/service.management.readonly,https://www.googleapis.com/auth/trace.append \\
--enable-display-device --tags=http-server,https-server \\
--create-disk=auto-delete=yes,boot=yes,device-name=free-tier-template,image=projects/debian-cloud/global/images/debian-11-bullseye-v20220719,mode=rw,size=30,type=pd-standard 
--no-shielded-secure-boot --shielded-vtpm --shielded-integrity-monitoring --reservation-affinity=any
```

##### ...or using Cloud console

- create an e2 in us-central1 (Iowa) for both zone and region
- select e2-micro (0.25-2 vCPU 1GB memory)
- you can change the boot disc from 10GB to 30GB if you like
- allow HTTPS and HTTP (if you need it for certificate provising)
- all else as default

### https certificate setup

#### Self signed (useful for localhost testing)

- You can use the bash script ```certs/gen.sh``` to generate a key/cert pair with openssl

#### Production; from authority

- get a domain (e.g. from squarespace)
- create a custom DNS record, e.g.
    - ```your.domain.somwhere    A	1 hour	google.cloud.instance.ip ```
- Use [Let's Encrypts](https://letsencrypt.org/) recommendation of [certbot](https://certbot.eff.org/) it really is very easy
    - You will need to enable http in the cloud instance firewall for provisioning as well as https
