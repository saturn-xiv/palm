#include "palm/utils.hpp"

#include <linux/watchdog.h>
#include <signal.h>
#include <sys/ioctl.h>
#include <sys/reboot.h>
#include <unistd.h>

void palm::reboot() {
  BOOST_LOG_TRIVIAL(warning) << "reboot";
  ::sync();
  ::reboot(RB_AUTOBOOT);
}

// https://www.kernel.org/doc/Documentation/watchdog/watchdog-api.txt
palm::WatchDog::WatchDog() {
  this->fd = ::open("/dev/watchdog", O_WRONLY);
  if (this->fd == -1) {
    throw std::invalid_argument("failed to open watchdog");
  }
}
palm::WatchDog::~WatchDog() { ::close(this->fd); }
void palm::WatchDog::feed() { ::ioctl(this->fd, WDIOC_KEEPALIVE, 0); }
int palm::WatchDog::get_timeout() {
  int v;
  ::ioctl(this->fd, WDIOC_SETTIMEOUT, &v);
  return v;
}
void palm::WatchDog::set_timeout(const int seconds) {
  BOOST_LOG_TRIVIAL(info) << "set watchdog's timeout to " << seconds;
  ::ioctl(fd, WDIOC_SETTIMEOUT, &seconds);
}
