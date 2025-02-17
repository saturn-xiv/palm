# DAHLIA - A totally open-source education solution.

## Usage

### PostgreSQL

- Create a database

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE dahlia_dev WITH ENCODING = 'UTF8' OWNER www;
```

```bash
~/local/liquibase-4.29.2/liquibase --version
~/local/liquibase-4.29.2/liquibase --defaults-file=liquibase-postgresql.properties connect | status | rollback-count --count 1 | history
```

### Build

```bash
gradle build -x test
java -jar build/libs/dahlia-2025.2.15-SNAPSHOT.jar
```
