#include "palm/utils.hpp"

#include <unistd.h>
#include <sys/reboot.h>

void palm::reboot(){
  ::sync();
  ::setuid(0);
  if(EXIT_SUCCESS != ::reboot(RB_AUTOBOOT)){
    spdlog::error("failed to reboot system");
  }
}
