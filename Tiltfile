load('ext://dotenv', 'dotenv')
dotenv(fn='.env.local')

local_resource('api', serve_cmd='cargo leptos watch')
# docker_build('api', '.', dockerfile='configs/docker/Dockerfile')
# k8s_yaml('configs/deployments/api.yaml')

k8s_yaml('configs/deployments/postgres.yaml')
# k8s_yaml('configs/deployments/prometheus.yaml')