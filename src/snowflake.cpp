#include "palm/snowflake.hpp"

void palm::Snowflake::set(const uint8_t datacenter_id,
                          const uint8_t worker_id) {
  if (worker_id > MAX_WORKER_ID) {
    std::stringstream ss;
    ss << "worker id can't be greater than " << MAX_WORKER_ID;
    throw std::runtime_error(ss.str());
  }

  if (datacenter_id > MAX_DATACENTER_ID) {
    std::stringstream ss;
    ss << "datacenter id can't be greater than " << MAX_DATACENTER_ID;
    throw std::runtime_error(ss.str());
  }
  {
    const std::lock_guard<std::mutex> lock(this->locker);
    this->datacenter_id = datacenter_id;
    this->worker_id = worker_id;
  }
}
