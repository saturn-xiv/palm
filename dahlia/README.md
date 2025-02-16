# DAHLIA - A totally open-source education solution.

## Usage

### PostgreSQL

- Create a database

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE dahlia_dev WITH ENCODING = 'UTF8' OWNER www;
```

```bash
~/local/flyway-11.3.2/flyway -configFiles=flyway-postgresql.conf info
```

### Build

```bash
gradle build -x test
java -jar build/libs/dahlia-2025.2.15-SNAPSHOT.jar
```
