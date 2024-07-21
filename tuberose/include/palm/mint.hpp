#pragma once

#include "palm/orm.hpp"

namespace palm {
namespace mint {

struct Migration {
  std::string name;
  std::string id;
  std::string up;
  std::string down;
  std::optional<std::string> run_at;
  std::string created_at;
};

class Driver {
 public:
  virtual void migrate(const std::string& id, const std::string& name,
                       const std::string& sql) = 0;
  virtual void rollback(const std::string& id, const std::string& name,
                        const std::string& sql) = 0;
  virtual void create() = 0;
  virtual void drop() = 0;
  virtual std::vector<Migration> status() = 0;
};

class PostgreSql : public Driver {
 public:
  PostgreSql(std::shared_ptr<pqxx::connection> db) : _db(db) {}
  void migrate(const std::string& id, const std::string& name,
               const std::string& sql) override;
  void rollback(const std::string& id, const std::string& name,
                const std::string& sql) override;
  void create() override;
  void drop() override;
  std::vector<Migration> status() override;

 private:
  std::shared_ptr<pqxx::connection> _db;
};

class MySql : public Driver {
 public:
  MySql(MYSQL* db) : _db(db) {}
  void migrate(const std::string& id, const std::string& name,
               const std::string& sql) override;
  void rollback(const std::string& id, const std::string& name,
                const std::string& sql) override;
  void create() override;
  void drop() override;
  std::vector<Migration> status() override;

 private:
  MYSQL* _db;
};

class Sqlite3 : public Driver {
 public:
  Sqlite3(std::shared_ptr<SQLite::Database> db) : _db(db) {}

  void migrate(const std::string& id, const std::string& name,
               const std::string& sql) override;
  void rollback(const std::string& id, const std::string& name,
                const std::string& sql) override;
  void create() override;
  void drop() override;
  std::vector<Migration> status() override;

 private:
  std::shared_ptr<SQLite::Database> _db;
};

class Schema {
 public:
  Schema(const std::filesystem::path& config,
         const std::filesystem::path& migrations);

  std::vector<Migration> status();
  void migrate(const std::string& id);
  void rollback(const std::string& id);

 private:
  std::shared_ptr<Driver> driver;
};

class Application {
 public:
  Application(int argc, char** argv);

 private:
  void generate(const std::filesystem::path& migrations,
                const std::string& name) const;
  void reset() const;
  std::vector<Migration> load(const std::filesystem::path& root) const;
};
}  // namespace mint
}  // namespace palm
