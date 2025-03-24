Proper manifest files for a deployment using Kubernetes. Obviously feel free to adjust to your needs. In this example, Traefik is used but feel free to use whatever ingress method (Nginx, HA Proxy, etc.) you would like, or create the Kubernetes secret via the CLI.

1. After deployed, enter the Kubernetes pod via a command such as "kubectl exec -it mollysocket-5c767fb96d-8gfzz -n default -- /bin/sh"

2. Generate the VAPID key in the pod by running the command "mollysocket vapid gen"

3. Copy the VAPID key from the prior command and paste into the secret.yaml file, under the ENV variable/key of "vapid_privkey".

Restart the Mollysocket pod, and you should be good to go!

