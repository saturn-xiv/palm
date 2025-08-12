#pragma once

#include "phlox/systemd.hpp"

namespace phlox {
namespace docker {
namespace models {
struct Container {
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

}  // namespace docker
}  // namespace phlox
