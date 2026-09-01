# Nginx with files watcher

Alpine-based docker image to watch directory and restart nginx when file was changed. Primary it was
created to update ssl certificates but you can use it for any other directory.

All features from [original docker image](https://hub.docker.com/_/nginx) work as well.

You must pass `WATCHER_DIRECTORIES` env variable to use the image. The variable supports multiple
folders separated by comma (with or without space):

```sh
WATCHER_DIRECTORIES="/etc/nginx/certs, /another/folder"
```

## Example

`docker-compose.yml`

```yml
services:
  nginx:
    image: ghcr.io/ym-project/nginx-with-files-watcher:v0.1.0
    volumes:
      # Pass certificates to image
      - certs-nginx-vol:/etc/nginx/certs:ro
    environment:
      # Waiting for folder changes
      WATCHER_DIRECTORIES: /etc/nginx/certs

volumes:
  certs-nginx-vol:
```
