# Usage

```bash
sudo apt install podman buildah runc
# first time start
./docker/ubuntu/first.sh
# next time start
./docker/ubuntu/next.sh
# start nginx/php-fpm/postgresql/mysql/rabbitmq/redis/elasticsearch/minio server
> sudo supervisord -c /etc/supervisor/supervisord.conf -n
# setup redis clusters
> ./docker/ubuntu/redis/setup.sh
# test redis connections
> redis-cli -p 6371
# enable rabbitmq Management Plugin 
> sudo rabbitmq-plugins enable rabbitmq_management
# test postgresql connection
> psql -U postgres -h 127.0.0.1 -p 5432
# setup mysql password
> sudo mysql_secure_installation
# test mysql connection
> mysql -h 127.0.0.1 -u root -p
```
