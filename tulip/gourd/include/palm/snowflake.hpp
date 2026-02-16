#pragma once

#include <spdlog/spdlog.h>

#include <chrono>
#include <iostream>
#include <mutex>

namespace palm {
class Snowflake {
 public:
  Snowflake(int64_t node_id)
      : _node_id(node_id), _last_timestamp(-1), _sequence(0) {}
  ~Snowflake() {}
  inline int64_t next() {
    std::lock_guard<std::mutex> guard(this->_mutex);

    uint64_t current_timestamp = Snowflake::timestamp();
    if (this->_last_timestamp == current_timestamp) {
      this->_sequence = (this->_sequence + 1) & SEQUENCE_MASK;
      if (this->_sequence == 0) {
        while (current_timestamp < this->_last_timestamp) {
          current_timestamp = Snowflake::timestamp();
        }
      }
    } else {
      this->_sequence = 0;
    }
    this->_last_timestamp = current_timestamp;
    int64_t id = static_cast<int64_t>(current_timestamp - EPOCH)
                     << TIMESTAMP_SHIFT |
                 (this->_node_id << NODE_ID_SHIFT) | this->_sequence;
    return id;
  }

 private:
  inline static uint64_t timestamp() {
    auto now = std::chrono::system_clock::now().time_since_epoch();
    return std::chrono::duration_cast<std::chrono::milliseconds>(now).count();
  }

 private:
  std::mutex _mutex;

  static const uint64_t EPOCH = 1771227731188;
  static const uint32_t NODE_ID_BITS = 10;
  static const uint32_t SEQUENCE_BITS = 12;
  static const uint16_t NODE_ID_SHIFT = SEQUENCE_BITS;
  static const uint16_t TIMESTAMP_SHIFT = SEQUENCE_BITS + NODE_ID_BITS;
  static const int64_t SEQUENCE_MASK = -1 ^ (-1 << SEQUENCE_BITS);

  int64_t _node_id;
  uint64_t _last_timestamp;
  int64_t _sequence;
};

}  // namespace palm
