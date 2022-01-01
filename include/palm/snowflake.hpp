#pragma once

#include <chrono>
#include <cstdint>
#include <mutex>
#include <sstream>
#include <stdexcept>

namespace palm {

class Snowflake {
 public:
  Snowflake(Snowflake const&) = delete;
  void operator=(Snowflake const&) = delete;

  static Snowflake& instance() {
    static Snowflake it;
    return it;
  }
  inline int64_t next() {
    const std::lock_guard<std::mutex> lock(this->locker);

    auto it = this->millsecond();
    if (this->last_timestamp == it) {
      this->sequence = (this->sequence + 1) & SEQUENCE_MASK;
      if (this->sequence == 0) {
        it = this->wait_next_millis(this->last_timestamp);
      }
    } else {
      this->sequence = 0;
    }

    this->last_timestamp = it;

    return ((it - this->started_epoch) << TIMESTAMP_LEFT_SHIFT) |
           (this->datacenter_id << DATACENTER_ID_SHIFT) |
           (this->worker_id << WORKER_ID_SHIFT) | this->sequence;
  }
  // FIXME
  // void set(const std::chrono::year_month_day& begin,
  //          const uint8_t datacenter_id, const uint8_t worker_id);

 private:
  Snowflake()
      : worker_id(1),
        datacenter_id(1),
        last_timestamp(0),
        sequence(0),
        started_time_point(std::chrono::steady_clock::now()),
        started_millsecond(
            std::chrono::duration_cast<std::chrono::milliseconds>(
                std::chrono::system_clock::now().time_since_epoch())
                .count()) {}

  inline int64_t millsecond() const noexcept {
    auto it = std::chrono::duration_cast<std::chrono::milliseconds>(
        std::chrono::steady_clock::now() - this->started_time_point);
    return this->started_millsecond + it.count();
  }

  inline int64_t wait_next_millis(int64_t last) const noexcept {
    auto it = this->millsecond();
    while (it <= last) {
      it = this->millsecond();
    }
    return it;
  }

  uint8_t worker_id;
  uint8_t datacenter_id;
  int64_t started_epoch;
  uint64_t last_timestamp;
  uint16_t sequence;
  int64_t started_millsecond;
  std::chrono::time_point<std::chrono::steady_clock> started_time_point;
  std::mutex locker;

  static constexpr int64_t WORKER_ID_BITS = 5L;
  static constexpr int64_t DATACENTER_ID_BITS = 5L;
  static constexpr int64_t SEQUENCE_BITS = 12L;
  static constexpr int64_t MAX_WORKER_ID = (1 << WORKER_ID_BITS) - 1;
  static constexpr int64_t MAX_DATACENTER_ID = (1 << DATACENTER_ID_BITS) - 1;
  static constexpr int64_t WORKER_ID_SHIFT = SEQUENCE_BITS;
  static constexpr int64_t DATACENTER_ID_SHIFT = SEQUENCE_BITS + WORKER_ID_BITS;
  static constexpr int64_t TIMESTAMP_LEFT_SHIFT =
      SEQUENCE_BITS + WORKER_ID_BITS + DATACENTER_ID_BITS;
  static constexpr int64_t SEQUENCE_MASK = (1 << SEQUENCE_BITS) - 1;
};

}  // namespace palm
