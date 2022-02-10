# imgBot

A discord bot focused on image manipulation and entertainment. Everything runs on K8s.

## Local run
Both the bot and server should run fine on local machines, just with environment vars set up.
Make sure to set the working directory to a temporary folder to decrease junk files piling up.

## K8s run
First, before deploying anything, add the imgbot namespace for convenience:


Before anything else, apply the main imgbot.yaml file.
```bash

kubectl apply -f kube/imgbot.yaml
```

Now, add bot token and appid as a secret:
```bash
kubectl create secret generic imgbot-secret \
  --namespace=imgbot \
  --from-literal=token='INSERT DISCORD TOKEN' \
  --from-literal=appid='INSERT APPID HERE'
```

Optionally, feel free to add a Tenor API key:
```bash
kubectl create secret generic imgbot-secret-tenor \
  --namespace=imgbot \
  --from-literal=apikey='INSERT TENOR API KEY' 
```

Then, check rollout of both deployments:
```bash
kubectl rollout status --namespace=imgbot deployment/imgserver
kubectl rollout status --namespace=imgbot deployment/imgbot
```

To check if everything is running correctly:
```bash
kubectl get pods --namespace=imgbot -o wide
```

There should be 5 image server instances and 1 bot instance alive. 