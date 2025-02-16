# DAHLIA - A totally open-source education solution.

## Usage

### PostgreSQL

- Create a database

```sql
CREATE USER user-name WITH PASSWORD 'change-me';
CREATE DATABASE db-name WITH ENCODING = 'UTF8' OWNER user-name;
```

```bash
~/local/flyway-11.3.2/flyway -configFiles=flyway-postgresql.conf info
```

### Build

```bash
gradle build -x test
```
