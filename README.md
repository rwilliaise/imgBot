# imgBot

A discord bot focused on image manipulation and entertainment. Everything runs on K8s.

## Local run
Both the bot and server should run fine on local machines, just with environment vars set up.
Make sure to set the working directory to a temporary folder to decrease junk files piling up.

## K8s run
First, before deploying anything, add the imgbot namespace for convenience:
```bash
kubectl apply -f kube/namespace-imgbot.yaml
```

First, before deploying anything, add bot token and appid as a secret:
```bash
kubectl create secret generic imgbot-secret \
  --namespace=imgbot \
  --from-literal=token='INSERT DISCORD TOKEN' \
  --from-literal=appid='INSERT APPID HERE'
```

Now, deploy all image servers:
```bash
kubectl apply -f kube/deploy-imgserver.yaml
kubectl rollout status --namespace=imgbot deployment/img-server

kubectl apply -f kube/service-imgserver.yaml
```

Then, deploy the main bot instances:
```bash
kubectl apply -f kube/deploy-imgbot.yaml
kubectl rollout status --namespace=imgbot  deployment/img-bot
```

To check if everything is running correctly:
```bash
kubectl get pods --namespace=imgbot -o wide
```

There should be 5 image server instances and 1 bot instance alive. 