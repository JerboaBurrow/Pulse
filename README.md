on local machine (with 3030 open)

````
switchboard -p 3030
```

on gcloud server
```
curl -i -H "Accept: application/json" -H "Content-Type:application/json" -X POST --data "dasflkjsdalkfjlasd;fjas" 81.78.215.199:3030
```

gave the reflection 

```
You sent:
dasflkjsdalkfjlasd;fjas
```

also worked via http github webhook

reqwest - worked posting to discord (https)

self signed cert generation

```
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 365
```