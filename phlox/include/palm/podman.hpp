#pragma once

#include "palm/systemd.hpp"

namespace palm {
namespace podman {
namespace models {
// https://www.freedesktop.org/software/systemd/man/latest/systemd.journal-fields.html
// journalctl --output=json-pretty -n 20 CONTAINER_NAME=xxx
struct Log {
  std::string _HOSTNAME;
  std::string _MACHINE_ID;
  std::string __SEQNUM;
  std::string __SEQNUM_ID;
  std::string __REALTIME_TIMESTAMP;
  palm::systemd::models::journal::Message MESSAGE;
  std::string CONTAINER_ID;
  std::string CONTAINER_ID_FULL;
  std::string CONTAINER_NAME;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Log, _HOSTNAME, _MACHINE_ID, __SEQNUM,
                                   __SEQNUM_ID, __REALTIME_TIMESTAMP, MESSAGE,
                                   CONTAINER_ID, CONTAINER_ID_FULL,
                                   CONTAINER_NAME)

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
  int64_t ExitedAt;
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
  int64_t StartedAt;
  std::string State;
  std::string Status;
  int64_t Created;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Item, AutoRemove, Command, CreatedAt,
                                   CIDFile, Exited, ExitedAt, ExitCode, Id,
                                   Image, ImageID, IsInfra, Labels, Mounts,
                                   Names, Pid, Pod, PodName, Ports, Restarts,
                                   StartedAt, State, Status, Created)
}  // namespace container
}  // namespace models

std::vector<models::Log> logs(const std::string& container_id, time_t since,
                              time_t until);
std::vector<models::Status> stats(bool all = false);
std::vector<models::container::Item> ps(bool all = false);
}  // namespace podman
}  // namespace palm
