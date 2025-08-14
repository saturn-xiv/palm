#include "aloe/utils.hpp"

#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <unistd.h>
#include <cstring>
#include <vector>

#include <spdlog/spdlog.h>

bool aloe::tcp(const std::string& host, uint8_t port) {
  int sock = socket(AF_INET, SOCK_STREAM, 0);
  if (sock == -1) {
    spdlog::error("socket creation failed");
    return false;
  }

  sockaddr_in server_addr;
  memset(&server_addr, 0, sizeof(server_addr));
  server_addr.sin_family = AF_INET;
  server_addr.sin_port = htons(port);
  server_addr.sin_addr.s_addr = inet_addr(host.c_str());

  bool ok;
  if (connect(sock, (sockaddr*)&server_addr, sizeof(server_addr)) == -1) {
    spdlog::error("connection tcp://{}:{} failed", host, port);
    ok = false;
  } else {
    ok = true;
  }

  close(sock);
  return ok;
}

void aloe::keep(const std::filesystem::path& target, const size_t count) {
  // TODO
  std::vector<std::filesystem::path> items;
  for (const auto& it : std::filesystem::directory_iterator(target)) {
    const auto file = it.path();
    if (std::filesystem::is_regular_file(file)) {
      spdlog::debug("find file {}", file.string());
      items.push_back(file);
    }
  }
  std::sort(items.begin(), items.end(),
            [](const auto& a, const auto& b) -> bool {
              return std::filesystem::last_write_time(a) >
                     std::filesystem::last_write_time(b);
            });

  spdlog::info("find {} backup files", items.size());
  if (items.size() > count) {
    for (auto it = items.begin() + count; it != items.end(); ++it) {
      spdlog::warn("remove file {}", it->string());
      std::filesystem::remove(*it);
    }
  }
}
