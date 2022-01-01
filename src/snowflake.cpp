#include "palm/snowflake.hpp"

// FIXME
// void palm::Snowflake::set(const std::chrono::year_month_day& begin,
//                           const uint8_t datacenter_id,
//                           const uint8_t worker_id) {
//   const std::chrono::time_point<std::chrono::system_clock> now{
//       std::chrono::system_clock::now()};

//   const std::chrono::year_month_day today{
//       std::chrono::floor<std::chrono::days>(now)};
//   if (!begin.ok() || begin > today) {
//     std::stringstream ss;
//     ss << "not a valid year-month-day ";
//     throw std::runtime_error(ss.str());
//   }

//   if (worker_id > MAX_WORKER_ID) {
//     std::stringstream ss;
//     ss << "worker id can't be greater than " << MAX_WORKER_ID;
//     throw std::runtime_error(ss.str());
//   }

//   if (datacenter_id > MAX_DATACENTER_ID) {
//     std::stringstream ss;
//     ss << "datacenter id can't be greater than " << MAX_DATACENTER_ID;
//     throw std::runtime_error(ss.str());
//   }
//   {
//     const std::lock_guard<std::mutex> lock(this->locker);

//     this->started_epoch =
//     std::chrono::duration_cast<std::chrono::milliseconds>(
//                               std::chrono::sys_days{begin}.time_since_epoch())
//                               .count();
//     this->datacenter_id = datacenter_id;
//     this->worker_id = worker_id;
//   }
// }
