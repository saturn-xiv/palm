// #pragma once

// #include "palm/pool.hpp"

// #include <boost/property_tree/ptree.hpp>

// #include <soci/soci.h>

// namespace palm {

// // .show Displays current settings for various parameters
// // .databases Provides database names and files
// // .quit Quit sqlite3 program
// // .tables Show current tables
// // .schema Display schema of table
// // .header Display or hide the output table header
// // .mode Select mode for the output table
// // .dump Dump database in SQL text format
// // pragma compile_options;
// // SELECT name FROM sqlite_master WHERE type='table' AND name='TABLE_NAME'
// namespace sqlite3 {
// void init();
// std::shared_ptr<soci::session> open(
//     const std::filesystem::path& file, bool wal_mode = false,
//     const std::chrono::seconds& busy_timeout = std::chrono::seconds{5});
// }  // namespace sqlite3

// // https://www.postgresql.org/docs/current/runtime-config-logging.html
// // /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
// // sudo journalctl -f -u postgresql
// namespace postgresql {
// void init();
// class Config {
//  public:
//   Config(const std::string& host = "127.0.0.1", uint16_t port = 5432,
//          const std::string& name = "dev", const std::string& user =
//          "postgres", const std::optional<std::string>& password =
//          std::nullopt, size_t size = 32)
//       : host(host),
//         port(port),
//         name(name),
//         user(user),
//         password(password),
//         size(size) {}

//   friend std::ostream& operator<<(std::ostream& out, const Config& self) {
//     return out << "postgresql://" << self.user << "@" << self.host << ":"
//                << self.port << "/" << self.name;
//   }

//   void operator=(const toml::table& node) {
//     {
//       auto it = node["host"].value<std::string>();
//       if (it) {
//         this->host = it.value();
//       }
//     }
//     {
//       auto it = node["port"].value<uint16_t>();
//       if (it) {
//         this->port = it.value();
//       }
//     }
//     {
//       auto it = node["name"].value<std::string>();
//       if (it) {
//         this->name = it.value();
//       }
//     }
//     {
//       auto it = node["user"].value<std::string>();
//       if (it) {
//         this->user = it.value();
//       }
//     }
//     this->password = node["password"].value<std::string>();
//     {
//       auto it = node["pool-size"].value<uint16_t>();
//       if (it) {
//         this->size = static_cast<size_t>(it.value());
//       }
//     }
//   }
//   friend toml::table& operator<<(toml::table& node, const Config& self) {
//     node.insert_or_assign("host", self.host);
//     node.insert_or_assign("port", self.port);
//     node.insert_or_assign("name", self.name);
//     node.insert_or_assign("user", self.user);
//     if (self.password) {
//       node.insert_or_assign("password", self.password.value());
//     }
//     node.insert_or_assign("pool-size", static_cast<uint8_t>(self.size));

//     return node;
//   }

//   std::shared_ptr<soci::session> build();

//   friend class palm::pool::Pool<soci::session, Config>;

//  private:
//   std::string host;
//   uint16_t port;
//   std::string user;
//   std::optional<std::string> password;
//   std::string name;
//   std::size_t size;
// };
// using Pool = palm::pool::Pool<soci::session, Config>;
// using PooledConnection = palm::pool::PooledConnection<Pool, soci::session>;

// }  // namespace postgresql

// // use DB-NAME
// // show tables;
// // desc TABLE-NAME;
// // SELECT table_name FROM information_schema.tables WHERE table_schema =
// // 'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
// namespace mysql {
// void init();
// class Config {
//  public:
//   Config(const std::string& host = "127.0.0.1", uint16_t port = 3306,
//          const std::string& name = "dev", const std::string& user = "root",
//          const std::optional<std::string>& password = std::nullopt,
//          size_t size = 32)
//       : host(host),
//         port(port),
//         name(name),
//         user(user),
//         password(password),
//         size(size) {}

//   friend std::ostream& operator<<(std::ostream& out, const Config& self) {
//     return out << "mysql://" << self.user << "@" << self.host << ":"
//                << self.port << "/" << self.name;
//   }

//   void operator=(const toml::table& node) {
//     {
//       auto it = node["host"].value<std::string>();
//       if (it) {
//         this->host = it.value();
//       }
//     }
//     {
//       auto it = node["port"].value<uint16_t>();
//       if (it) {
//         this->port = it.value();
//       }
//     }
//     {
//       auto it = node["name"].value<std::string>();
//       if (it) {
//         this->name = it.value();
//       }
//     }
//     {
//       auto it = node["user"].value<std::string>();
//       if (it) {
//         this->user = it.value();
//       }
//     }
//     this->password = node["password"].value<std::string>();
//     {
//       auto it = node["pool-size"].value<uint16_t>();
//       if (it) {
//         this->size = static_cast<size_t>(it.value());
//       }
//     }
//   }
//   friend toml::table& operator<<(toml::table& node, const Config& self) {
//     node.insert_or_assign("host", self.host);
//     node.insert_or_assign("port", self.port);
//     node.insert_or_assign("name", self.name);
//     node.insert_or_assign("user", self.user);
//     if (self.password) {
//       node.insert_or_assign("password", self.password.value());
//     }
//     node.insert_or_assign("pool-size", static_cast<uint8_t>(self.size));

//     return node;
//   }

//   std::shared_ptr<soci::session> build();

//   friend class palm::pool::Pool<soci::session, Config>;

//  private:
//   std::string host;
//   uint16_t port;
//   std::string user;
//   std::optional<std::string> password;
//   std::string name;
//   std::size_t size;
// };
// using Pool = palm::pool::Pool<soci::session, Config>;
// using PooledConnection = palm::pool::PooledConnection<Pool, soci::session>;
// }  // namespace mysql

// namespace orm {
// class Migration {
//  public:
//   Migration() {}
//   Migration(const std::filesystem::path& root);

//   static void generate(const std::filesystem::path& root,
//                        const std::string& name);

//   static void header(std::ostream& out) {
//     std::ios_base::fmtflags f(out.flags());
//     out << std::left << std::setw(Migration::VERSION_SIZE) << "Version"
//         << std::setw(Migration::NAME_SIZE) << "Name"
//         << std::setw(Migration::RUN_AT_SIZE) << "Run At";
//     out.flags(f);
//   }

//   friend std::ostream& operator<<(std::ostream& out, Migration const& self) {
//     std::ios_base::fmtflags f(out.flags());
//     out << std::left << std::setw(Migration::VERSION_SIZE) << self.version
//         << std::setw(Migration::NAME_SIZE) << self.name
//         << std::setw(Migration::RUN_AT_SIZE);
//     if (self.run_at) {
//       out << std::asctime(&self.run_at.value());
//     } else {
//       out << "n/a";
//     }
//     out.flags(f);
//     return out;
//   }

//   friend class Schema;

//   friend struct soci::type_conversion<Migration>;

//  private:
//   int32_t id;
//   std::string name;
//   std::string up;
//   std::string down;
//   std::string version;
//   std::optional<std::tm> run_at;
//   std::tm created_at;

//   inline static const std::string DIV = "-";
//   inline static const std::string UP = "up.sql";
//   inline static const std::string DOWN = "down.sql";
//   inline static const std::string MIGRATION_FOLDER = "migrations";

//   static const int VERSION_SIZE = 15;
//   static const int NAME_SIZE = 36;
//   static const int RUN_AT_SIZE = 24;
// };

// class Logger : public soci::logger_impl {
//  public:
//   void start_query(std::string const& query) override {
//     BOOST_LOG_TRIVIAL(debug) << query;
//   }

//  private:
//   logger_impl* do_clone() const override { return new Logger(); }
// };

// class Schema {
//  public:
//   Schema(std::shared_ptr<soci::session> db, std::filesystem::path& root);

//   void migrate();
//   void rollback();
//   void status();

//   static void load_queries(const std::filesystem::path& root);
//   inline static std::string query(const std::string& name) {
//     return QUERIES.get<std::string>(name);
//   }

//  private:
//   void load_migrations();

//   std::mutex mutex;
//   std::shared_ptr<soci::session> db;
//   std::filesystem::path& root;

//   inline static boost::property_tree::ptree QUERIES;
// };

// }  // namespace orm
// }  // namespace palm

// namespace soci {

// template <>
// struct type_conversion<palm::orm::Migration> {
//   typedef values base_type;
//   static void from_base(values const& v, soci::indicator,
//                         palm::orm::Migration& p) {
//     p.id = v.get<int32_t>("id");
//     p.name = v.get<std::string>("name");
//     p.version = v.get<std::string>("version");
//     p.up = v.get<std::string>("up");
//     p.down = v.get<std::string>("down");
//     if (v.get_indicator("run_at") == i_ok) {
//       p.run_at = v.get<std::tm>("run_at");
//     } else {
//       p.run_at = std::nullopt;
//     }

//     p.created_at = v.get<std::tm>("created_at");
//   }
//   static void to_base(const palm::orm::Migration& p, soci::values& v,
//                       soci::indicator& ind) {
//     v.set("id", p.id);
//     v.set("name", p.name);
//     v.set("version", p.version);
//     v.set("up", p.up);
//     v.set("down", p.down);
//     if (p.run_at) {
//       v.set("run_at", p.run_at.value());
//     } else {
//       v.set("run_at", NULL);
//     }
//     v.set("created_at", p.created_at);
//     ind = i_ok;
//   }
// };

// }  // namespace soci
