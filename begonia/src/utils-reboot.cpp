#include "palm/utils.hpp"

#include <unistd.h>
#include <sys/reboot.h>

void palm::reboot(){
  sync();
  reboot(RB_AUTOBOOT);
}
