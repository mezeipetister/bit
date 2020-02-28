# Docker should be empty of webserver and bit
# This script creates webserver and bit containers
# from webserver and bit images.
# Sets data mount, env variable mount set the same network
# and set restart always.
docker pull mezeipetister/gnstore_client:latest
docker pull mezeipetister/gnstore:latest
docker stop client
docker rm client
docker run --name client --network host --restart always -d mezeipetister/gnstore_client:latest
docker stop api
docker rm api
docker run --name api --network host -v /data:/app/data --env-file /env/ENV.list --restart always -d mezeipetister/gnsotre:latest
