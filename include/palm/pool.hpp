// #pragma once

// #include "palm/env.hpp"

// namespace palm {
// namespace pool {

// template <class P, class C>
// class PooledConnection {
//  public:
//   PooledConnection(std::shared_ptr<P> pool, std::shared_ptr<C> instance)
//       : pool(pool), instance(instance) {}
//   ~PooledConnection() {
//     if (instance != nullptr) {
//       this->pool->release(this->instance);
//     }
//   }

//  private:
//   std::shared_ptr<P> pool;
//   std::shared_ptr<C> instance;
// };

// template <class C, class B>
// class Pool {
//  public:
//   Pool(std::shared_ptr<B> builder) : builder(builder) {
//     BOOST_LOG_TRIVIAL(debug) << "open pool " << (*this->builder)
//                              << " with size " << this->builder->size;
//     for (auto i = 0; i < this->builder->size; i++) {
//       auto it = this->builder->build();
//       this->queue.push(it);
//     }
//   }

//   std::shared_ptr<C> get() {
//     std::lock_guard<std::mutex> lock(this->mutex);
//     if (this->queue.empty()) {
//       BOOST_LOG_TRIVIAL(error) << "pool" << (*this->builder) << " is empty";
//       return nullptr;
//     }
//     BOOST_LOG_TRIVIAL(debug) << "get a " << (*this->builder) << "
//     connection"; auto it = this->queue.front(); this->queue.pop(); return it;
//   }

//   void release(std::shared_ptr<C> it) {
//     std::lock_guard<std::mutex> lock(this->mutex);
//     if (it != nullptr) {
//       BOOST_LOG_TRIVIAL(debug)
//           << "release a " << (*this->builder) << " connection";
//       this->queue.push(it);
//     }
//   }

//   void heartbeat(std::function<bool(std::shared_ptr<C>)> fn) {
//     std::lock_guard<std::mutex> lock(this->mutex);
//     std::queue<std::shared_ptr<C>> queue;

//     while (!this->queue.empty()) {
//       auto it = this->queue.front();
//       this->queue.pop();
//       if (!fn(it)) {
//         BOOST_LOG_TRIVIAL(error) << (*this->builder) << " heartbeat failed";
//         it = this->builder->build();
//       }
//       queue.push(it);
//     }
//     this->queue = queue;
//   }
//   inline size_t idle() {
//     std::lock_guard<std::mutex> lock(this->mutex);
//     return queue.size();
//   }

//  private:
//   std::mutex mutex;
//   std::queue<std::shared_ptr<C>> queue;
//   std::shared_ptr<B> builder;
// };

// }  // namespace pool
// }  // namespace palm
