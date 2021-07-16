# Usage

```bash
sudo apt install podman buildah runc
# first time start
./docker/ubuntu/first.sh
# next time start
./docker/ubuntu/next.sh
# start nginx/php-fpm/postgresql/rabbitmq/redis/elasticsearch server
> sudo supervisord -c /etc/supervisor/supervisord.conf
# redis
> redis-cli
# postgresql
> psql -U postgres -h 127.0.0.1 -p 5433 
```
