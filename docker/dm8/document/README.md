# DM8

## Start service

```bash
$ cd ~/tmp
$ ~/workspace/saturn-xiv/palm/docker/dm8/start.sh 15236 # listen on 0.0.0.0:15236

> /opt/bin/DmServiceDMSERVER start
> /opt/bin/DmServiceDMSERVER stop
> /opt/bin/DmServiceDMSERVER status
```

## Usage

```bash
$ cd /opt/dmdbms/bin/
$ ./disql # user: sysdba password: 123456789

SQL> SELECT * FROM V$VERSION;
SQL> SELECT SVR_VERSION, DB_VERSION, START_TIME, BUILD_TIME, BUILD_VERSION FROM V$INSTANCE;
SQL> SELECT EXPIRED_DATE FROM V$LICENSE;
```

- create user

```sql
-- create tablespace "demo" 10G
CREATE TABLESPACE "demo" DATAFILE '/var/lib/dm8/demo.dbf' SIZE 10240;

-- username: www password(at least 9 characters，MUST include upper/lower letters、 numbers、special symbols): change-ME@2024
CREATE USER "www" IDENTIFIED BY "change-ME@2024" HASH WITH SHA512 SALT ENCRYPT BY "123456" DEFAULT TABLESPACE "demo" DEFAULT INDEX TABLESPACE "demo";
GRANT "DBA" TO "www";
```

![dba](dba.png)

![user](user.png)

## DBeaver

- Add the jdbc driver

![1](dbeaver/driver-1.png)
![2](dbeaver/driver-2.png)
![3](dbeaver/driver-3.png)

- Create a connection

![1](dbeaver/connection-1.png)
![2](dbeaver/connection-2.png)
![3](dbeaver/connection-3.png)

- Test the connection

![1](dbeaver/create-table-1.png)
![2](dbeaver/create-table-2.png)
