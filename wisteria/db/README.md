# Database

- Setup database

  ```sql
  CREATE USER www WITH PASSWORD 'change-me';
  CREATE DATABASE wisteria_dev WITH OWNER www;
  ```

- Setup `.env` file

  ```bash
  DATABASE_URL="postgres://www:change-me@127.0.0.1:5432/wisteria_dev?sslmode=disable"
  DBMATE_MIGRATIONS_DIR="./migrations"
  ```
