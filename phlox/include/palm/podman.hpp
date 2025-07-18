#pragma once

#include "palm/theme.hpp"

// #include <chrono>
// #include <cstdint>
// #include <ctime>
// #include <map>
// #include <string>
// #include <vector>

// #include <nlohmann/json.hpp>

namespace palm {
namespace podman {
namespace models {
struct Log {
  // 2025-07-17T15:15:08.301003000Z
  std::string timestamp;
  std::string message;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Log, timestamp, message)

struct Status {
  std::string id;
  std::string name;
  std::string cpu_time;
  std::string cpu_percent;
  std::string avg_cpu;
  std::string mem_usage;
  std::string mem_percent;
  std::string net_io;
  std::string block_io;
  std::string pids;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Status, id, name, cpu_time, cpu_percent,
                                   avg_cpu, mem_usage, mem_percent, net_io,
                                   block_io, pids)

namespace container {
struct Port {
  std::string host_ip;
  uint16_t container_port;
  uint16_t host_port;
  int range;
  std::string protocol;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Port, host_ip, container_port, host_port,
                                   range, protocol)

struct Item {
  bool AutoRemove;
  std::vector<std::string> Command;
  std::string CreatedAt;
  std::string CIDFile;
  bool Exited;
  int ExitedAt;
  int ExitCode;
  std::string Id;
  std::string Image;
  std::string ImageID;
  bool IsInfra;
  std::map<std::string, std::string> Labels;
  std::vector<std::string> Mounts;
  std::vector<std::string> Names;
  //   TODO Namespaces
  //   TODO Networks
  int Pid;
  std::string Pod;
  std::string PodName;
  std::optional<std::vector<Port>> Ports;
  int Restarts;
  // TODO Size
  int StartedAt;
  std::string State;
  std::string Status;
  int Created;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Item, AutoRemove, Command, CreatedAt,
                                   CIDFile, Exited, ExitedAt, ExitCode, Id,
                                   Image, ImageID, IsInfra, Labels, Mounts,
                                   Names, Pid, Pod, PodName, Ports, Restarts,
                                   StartedAt, State, Status, Created)
}  // namespace container
}  // namespace models

std::vector<models::Log> logs(
    const std::string& container_id, std::tm* begin,
    const std::chrono::seconds ttl =
        std::chrono::duration_cast<std::chrono::seconds>(std::chrono::hours{
            1}));
std::vector<models::Status> stats();
std::vector<models::container::Item> ps();
}  // namespace podman
}  // namespace palm
