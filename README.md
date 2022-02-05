# imgBot

A discord bot focused on image manipulation and entertainment. Everything runs on K8s.

## Local run
Both the bot and server should run fine on local machines, just with environment vars set up.
Make sure to set the working directory to a temporary folder to decrease junk files piling up.

## K8s run
First, before deploying anything, add bot token and appid as a secret:
```bash
kubectl create secret generic imgbot-secret \
  --from-literal=token='INSERT DISCORD TOKEN'
  --from-literal=appid='INSERT APPID HERE'
```

Now, deploy all image servers:
```bash
kubectl apply -f kube/deploy-imgserver.yaml
kubectl rollout status deployment/img-server

kubectl apply -f kube/service-imgserver.yaml
```

Then, deploy the main bot instances:
```bash
kubectl apply -f kube/deploy-imgbot.yaml
kubectl rollout status deployment/img-bot
```

To check if everything is running correctly:
```bash
kubectl get pods -o wide
```

There should be 5 image server instances and 2 bot instances alive. 