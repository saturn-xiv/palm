#pragma once

#include <deque>
#include <exception>
#include <memory>
#include <mutex>
#include <optional>
#include <set>
#include <string>

#include <boost/log/trivial.hpp>

namespace palm {
namespace pool {
class Connection {
 public:
  Connection(){};
  virtual ~Connection(){};
};

class ConnectionFactory {
 public:
  virtual std::shared_ptr<Connection> create() = 0;
  virtual std::string name() = 0;
};
struct ConnectionPoolStats {
  size_t size;
  size_t idle;
};

template <class T>
class ConnectionPool {
 public:
  ConnectionPoolStats stats() {
    const std::lock_guard<std::mutex> lock(this->locker);
    ConnectionPoolStats it;
    it.size = this->pool.size();
    it.idle = it.size - this->borrowed.size();
    return stats;
  };

  ConnectionPool(size_t size, std::shared_ptr<ConnectionFactory> factory)
      : factory(factory) {
    while (this->pool.size() < size) {
      this->pool.push_back(this->factory->create());
    }
  };

  ~ConnectionPool(){};

  std::shared_ptr<T> borrow() {
    const std::lock_guard<std::mutex> lock(this->locker);

    if (this->pool.size() == 0) {
      for (auto& it : this->borrow) {
        if (it.unique()) {
          // TODO
          //   try {
          //     // If we are able to create a new connection, return it
          //     _DEBUG("Creating new connection to replace discarded
          //     connection"); shared_ptr<Connection> conn =
          //     this->factory->create(); this->borrowed.erase(it);
          //     this->borrowed.insert(conn);
          //     return boost::static_pointer_cast<T>(conn);

          //   } catch (std::exception& e) {
          //     // Error creating a replacement connection
          //     throw ConnectionUnavailable();
          //   }
        }
      }

      std::stringstream ss;
      ss << "no available " << this->factory->name() << "connection";
      throw std::runtime_error(ss.str());
    }

    std::shared_ptr<Connection> it = this->pool.front();
    this->pool.pop_front();
    this->borrowed.insert(it);
    return std::static_pointer_cast<T>(it);
  };

  void release(std::shared_ptr<T> it) {
    const std::lock_guard<std::mutex> lock(this->locker);
    this->pool.push_back(std::static_pointer_cast<Connection>(it));
    this->borrowed.erase(it);
  };

 protected:
  std::shared_ptr<ConnectionFactory> factory;

  std::deque<std::shared_ptr<Connection>> pool;
  std::set<std::shared_ptr<Connection>> borrowed;
  std::mutex locker;
};

}  // namespace pool
}  // namespace palm
