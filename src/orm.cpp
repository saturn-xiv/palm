// #include "palm/orm.hpp"

// #include <boost/property_tree/ini_parser.hpp>

// #include <soci/mysql/soci-mysql.h>
// #include <soci/postgresql/soci-postgresql.h>
// #include <soci/sqlite3/soci-sqlite3.h>

// void palm::sqlite3::init() { soci::register_factory_sqlite3(); }

// //
// https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of
// std::shared_ptr<soci::session> palm::sqlite3::open(
//     const std::filesystem::path& file, bool wal_mode,
//     const std::chrono::seconds& busy_timeout) {
//   std::stringstream ss;
//   ss << "sqlite3://" << file.string();
//   auto it = std::make_shared<soci::session>(ss.str());
//   {
//     if (wal_mode) {
//       (*it) << "PRAGMA synchronous = OFF";
//       (*it) << "PRAGMA journal_mode = WAL";
//     }
//     (*it) << "PRAGMA foreign_keys = ON";

//     {
//       std::stringstream ss;
//       ss << "PRAGMA busy_timeout = "
//          << std::chrono::milliseconds(busy_timeout).count();
//       (*it) << ss.str();
//     }
//   }
//   it->set_logger(new palm::orm::Logger());
//   return it;
// }

// void palm::postgresql::init() { soci::register_factory_postgresql(); }

// //
// https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PARAMKEYWORDS
// std::shared_ptr<soci::session> palm::postgresql::Config::build() {
//   std::stringstream ss;
//   ss << "postgresql://"
//      << "host=" << this->host << " port=" << this->port
//      << " dbname=" << this->name << " user=" << this->user;
//   if (this->password) {
//     ss << " password" << this->password.value();
//   }
//   ss << " sslmode=disable";

//   auto it = std::make_shared<soci::session>(ss.str());
//   it->set_logger(new palm::orm::Logger());
//   return it;
// }

// void palm::mysql::init() { soci::register_factory_mysql(); }

// // http://soci.sourceforge.net/doc/master/backends/mysql/
// std::shared_ptr<soci::session> palm::mysql::Config::build() {
//   std::stringstream ss;
//   ss << "mysql://"
//      << "host=" << this->host << " port=" << this->port
//      << " dbname=" << this->name << " user=" << this->user;
//   if (this->password) {
//     ss << " password" << this->password.value();
//   }

//   auto it = std::make_shared<soci::session>(ss.str());
//   it->set_logger(new palm::orm::Logger());
//   return it;
// }

// void palm::orm::Migration::generate(const std::filesystem::path& root,
//                                     const std::string& name) {
//   std::stringstream ss;
//   auto now = std::time(nullptr);
//   ss << std::put_time(std::gmtime(&now), "%Y%m%d%H%M%S") << Migration::DIV
//      << name;
//   const auto node = root / Migration::MIGRATION_FOLDER / ss.str();
//   BOOST_LOG_TRIVIAL(info) << "create migration in folder " << node;
//   std::filesystem::create_directories(node);
//   {
//     std::ofstream it(node / Migration::UP);
//     it.close();
//   }
//   {
//     std::ofstream it(node / Migration::DOWN);
//     it.close();
//   }
// }

// palm::orm::Migration::Migration(const std::filesystem::path& root) {
//   BOOST_LOG_TRIVIAL(info) << "load migration from " << root;
//   {
//     std::ifstream it(root / Migration::UP);
//     this->up = std::string((std::istreambuf_iterator<char>(it)),
//                            std::istreambuf_iterator<char>());
//     boost::trim(this->up);
//   }
//   {
//     std::ifstream it(root / Migration::DOWN);
//     this->down = std::string((std::istreambuf_iterator<char>(it)),
//                              std::istreambuf_iterator<char>());
//     boost::trim(this->down);
//   }

//   const auto fn = root.filename().string();
//   const auto pos = fn.find(Migration::DIV);
//   if (pos == std::string::npos) {
//     throw std::invalid_argument("bad folder name");
//   }
//   this->version = fn.substr(0, pos);
//   this->name = fn.substr(pos + 1);
// }

// void palm::orm::Schema::load_queries(const std::filesystem::path& root) {
//   auto file = root / "queries.ini";
//   BOOST_LOG_TRIVIAL(debug) << "load queries from " << file.string();
//   boost::property_tree::ini_parser::read_ini(file, QUERIES);
// }

// palm::orm::Schema::Schema(std::shared_ptr<soci::session> db,
//                           std::filesystem::path& root)
//     : db(db), root(root) {
//   std::ifstream it(this->root / "schema_migrations.sql");
//   std::string txt((std::istreambuf_iterator<char>(it)),
//                   std::istreambuf_iterator<char>());
//   boost::trim(txt);

//   soci::transaction tr(*(this->db));
//   (*(this->db)) << txt;
//   tr.commit();
// }

// void palm::orm::Schema::load_migrations() {
//   const auto node = this->root / Migration::MIGRATION_FOLDER;

//   BOOST_LOG_TRIVIAL(debug) << "load db migrations from " << node.string();

//   soci::transaction tr(*(this->db));
//   for (const auto& it : std::filesystem::directory_iterator(node)) {
//     if (std::filesystem::is_directory(it)) {
//       auto mig = Migration(it);
//       BOOST_LOG_TRIVIAL(debug)
//           << "find migration " << mig.version << " " << mig.name;
//       size_t c;
//       (*(this->db)) << Schema::query(
//           "schema_migrations.exists-by-version-and-name"),
//           soci::into(c), soci::use(mig.name, "name"),
//           soci::use(mig.version, "version");
//       if (c == 0) {
//         BOOST_LOG_TRIVIAL(info) << "not exists, will insert it";
//         (*(this->db)) << Schema::query("schema_migrations.insert"),
//             soci::use(mig.name, "name"), soci::use(mig.version, "version"),
//             soci::use(mig.up, "up"), soci::use(mig.down, "down");
//       }
//     }
//   }
//   tr.commit();
// }

// void palm::orm::Schema::migrate() {
//   std::lock_guard<std::mutex> lock(this->mutex);
//   this->load_migrations();

//   soci::rowset<Migration> rs =
//       (this->db->prepare << Schema::query("schema_migrations.all"));
//   for (soci::rowset<Migration>::const_iterator it = rs.begin(); it !=
//   rs.end();
//        ++it) {
//     if (!it->run_at) {
//       BOOST_LOG_TRIVIAL(info)
//           << "run migration " << it->version << Migration::DIV << it->name;
//       // std::tm* now = nullptr;
//       // {
//       //   std::time_t it = std::time(nullptr);
//       //   now = std::gmtime(&it);
//       // }
//       // soci::use(*now, "run_at"),
//       soci::transaction tr(*(this->db));
//       (*(this->db)) << it->up;
//       (*(this->db)) << Schema::query("schema_migrations.set-run-at"),
//           soci::use(it->id, "id");
//       tr.commit();
//     }
//   }
// }

// void palm::orm::Schema::rollback() {
//   std::lock_guard<std::mutex> lock(this->mutex);

//   Migration it;
//   soci::indicator ind;
//   (*(this->db)) << Schema::query("schema_migrations.latest"),
//       soci::into(it, ind);
//   if (ind == soci::indicator::i_null || !it.run_at) {
//     BOOST_LOG_TRIVIAL(warning) << "database is empty";
//   } else {
//     BOOST_LOG_TRIVIAL(info) << "rollback " << it.version << " " << it.name;
//     soci::transaction tr(*(this->db));
//     (*(this->db)) << it.down;
//     (*(this->db)) << Schema::query("schema_migrations.delete"),
//         soci::use(it.id, "id");
//     tr.commit();
//   }
// }

// void palm::orm::Schema::status() {
//   std::lock_guard<std::mutex> lock(this->mutex);
//   this->load_migrations();

//   soci::rowset<Migration> rs =
//       (this->db->prepare << Schema::query("schema_migrations.all"));
//   Migration::header(std::cout);
//   std::cout << std::endl;
//   std::copy(rs.begin(), rs.end(),
//             std::ostream_iterator<Migration>(std::cout, ""));
// }
