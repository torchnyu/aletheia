# Deployment Notes

These are notes in case we need to redeploy.

Traefik
	- It's a pain, but it's less of a pain than NGINX.
	- [Here's an up to date, good tutorial](https://www.digitalocean.com/community/tutorials/how-to-use-traefik-as-a-reverse-proxy-for-docker-containers-on-ubuntu-18-04)
	- `ports` works, `expose` doesn't. I don't know why.
	
Docker Compose
	- Sometimes it randomly hangs. This is for [weird reasons](https://github.com/docker/compose/issues/6678)
	- Run without `-d` to debug
	
Diesel
	- Oh man migrations are a pain. 
	- Right now we're just running migrations manually. For this to
      work, we need diesel_cli on the server, which also requires
      postgresql and libpq-dev. I know, not very reproducible, but the
      alternative involves complicated embedded migrations or exposing
      the db to the outside world.
