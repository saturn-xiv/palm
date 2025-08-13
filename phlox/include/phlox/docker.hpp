#pragma once

#include "phlox/podman.hpp"

namespace phlox {
namespace docker {
namespace models {
struct Container {
  inline time_t created_at() const {
    const std::string FORMAT = "%Y-%m-%d %H:%M:%S %z %Z";
    std::tm it = {0};
    {
      char* rst = strptime(this->CreatedAt.c_str(), FORMAT.c_str(), &it);
      if (rst == nullptr) {
        spdlog::error("parse tm failed({})", this->CreatedAt);
        return 0;
      }
    }
    time_t seconds = mktime(&it);
    return seconds;
  }
  std::string Command;
  std::string CreatedAt;
  std::string ID;
  std::string Image;
  std::string Labels;
  std::string LocalVolumes;
  std::string Mounts;
  std::string Names;
  std::string Networks;
  std::optional<std::string> Platform;
  std::string Ports;
  std::string RunningFor;
  std::string Size;
  std::string State;
  std::string Status;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Container, Command, CreatedAt, ID, Image,
                                   Labels, LocalVolumes, Mounts, Names,
                                   Networks, Platform, Ports, RunningFor, Size,
                                   State, Status)
struct Status {
  std::string BlockIO;
  std::string CPUPerc;
  std::string Container;
  std::string ID;
  std::string MemPerc;
  std::string MemUsage;
  std::string Name;
  std::string NetIO;
  std::string PIDs;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Status, BlockIO, CPUPerc, Container, ID,
                                   MemPerc, MemUsage, Name, NetIO, PIDs)
}  // namespace models

std::vector<models::Status> stats(bool all = false);
std::vector<models::Container> ps(bool all = false);
std::vector<phlox::podman::models::Log> logs(const std::string& container_id,
                                             time_t since, time_t until);

}  // namespace docker
}  // namespace phlox
