# Usage

```bash
sudo apt install podman buildah runc
# import image
podman load -q -i palm-TIMESTAMP.tar
# for test
podman run --rm -it --userns=keep-id --hostname=palm --user=$(id -ur):$(id -gr) --network host --events-backend=file -v $PWD:/workspace:z palm
# first time start
./docker/ubuntu/first.sh
# next time start
./docker/ubuntu/next.sh
# start nginx/php-fpm/postgresql/mysql/rabbitmq/redis/elasticsearch/minio server
> sudo supervisord -c /etc/supervisor/supervisord.conf -n
# check services
> supervisorctl status
# setup redis clusters
> ./docker/ubuntu/redis/setup.sh
# test redis connections
> redis-cli -p 6371
# enable rabbitmq Management Plugin: user`guest` password`guest` 
> sudo rabbitmq-plugins enable rabbitmq_management
# test postgresql connection
> psql -U postgres -h 127.0.0.1 -p 5432
# setup mysql password
> sudo mysql_secure_installation
# test mysql connection
> mysql -h 127.0.0.1 -u root -p
# test elasticsearch connection
> curl http://127.0.0.1:9200/
# test minio: user`admin` password`12345678`
> curl http://localhost:9001
```

## Issues

- [ERROR 1698 (28000): Access denied for user 'root'@'localhost'](https://stackoverflow.com/questions/39281594/error-1698-28000-access-denied-for-user-rootlocalhost)
