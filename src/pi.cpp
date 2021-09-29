// #include "palm/pi.hpp"

// #include <arpa/inet.h>
// #include <errno.h>
// #include <net/if.h>
// #include <netinet/in.h>
// #include <sys/ioctl.h>
// #include <sys/socket.h>
// #include <sys/sysinfo.h>
// #include <sys/types.h>
// #include <sys/utsname.h>
// #include <unistd.h>

// #include <boost/algorithm/string.hpp>
// #include <boost/log/trivial.hpp>

// #include <palm/utils.hpp>

// grpc::Status palm::pi::Service::SignIn(
//     grpc::ServerContext* context, const palm::pi::v1::SignInRequest* request,
//     palm::pi::v1::SignInResponse* response) {
//   // TODO
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::SignOut(grpc::ServerContext* context,
//                                         const google::protobuf::Empty*
//                                         request, google::protobuf::Empty*
//                                         response) {
//   // TODO
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::GetUser(grpc::ServerContext* context,
//                                         const google::protobuf::Empty*
//                                         request, palm::pi::v1::Administrator*
//                                         response) {
//   // TODO
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::SetUser(
//     grpc::ServerContext* context, const palm::pi::v1::Administrator* request,
//     google::protobuf::Empty* response) {
//   // TODO
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::Logs(grpc::ServerContext* context,
//                                      const google::protobuf::Empty* request,
//                                      palm::pi::v1::LogsResponse* response) {
//   // TODO
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::Status(grpc::ServerContext* context,
//                                        const google::protobuf::Empty*
//                                        request, palm::pi::v1::StatusResponse*
//                                        response) {
//   const Session ss(context);
//   if (!ss.current_user(this->jwt)) {
//     return grpc::Status(grpc::StatusCode::UNAUTHENTICATED, "");
//   }
//   struct utsname un;
//   {
//     if (uname(&un) < 0) {
//       return grpc::Status(grpc::StatusCode::INTERNAL, strerror(errno));
//     }
//     response->mutable_os()->set_domain_name(un.domainname);
//     response->mutable_os()->set_sys_name(un.sysname);
//     response->mutable_os()->set_node_name(un.nodename);
//     response->mutable_os()->set_machine(un.machine);
//     response->mutable_os()->set_release(un.release);
//     response->mutable_os()->set_version(un.version);
//   }
//   struct sysinfo si;
//   {
//     if (sysinfo(&si) < 0) {
//       return grpc::Status(grpc::StatusCode::INTERNAL, strerror(errno));
//     }
//     response->mutable_ram()->set_free(si.freeram);
//     response->mutable_ram()->set_max(si.totalhigh);
//     response->mutable_ram()->set_total(si.totalram);

//     response->mutable_swap()->set_free(si.freeswap);
//     response->mutable_swap()->set_total(si.totalswap);

//     response->mutable_load()->set_one(si.loads[0]);
//     response->mutable_load()->set_five(si.loads[1]);
//     response->mutable_load()->set_fifteen(si.loads[2]);

//     response->mutable_up_time()->set_seconds(si.uptime);
//   }
//   for (const auto& it :
//   std::filesystem::directory_iterator("/sys/class/net")) {
//     const auto root = it.path();
//     const auto name = root.filename();

//     std::ifstream sif(root / "address");
//     std::string mac((std::istreambuf_iterator<char>(sif)),
//                     std::istreambuf_iterator<char>());
//     boost::trim(mac);

//     auto item = response->add_networks();
//     item->set_mac(mac);
//     item->set_name(name);

//     {
//       int fd = ::socket(AF_INET, SOCK_DGRAM, 0);
//       struct ifreq ifr;
//       ifr.ifr_addr.sa_family = AF_INET;
//       ::strncpy(ifr.ifr_name, name.c_str(), IFNAMSIZ - 1);
//       ::ioctl(fd, SIOCGIFADDR, &ifr);
//       item->set_ip(::inet_ntoa(((struct
//       sockaddr_in*)&ifr.ifr_addr)->sin_addr));
//       ::close(fd);
//     }
//   }
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::GetEther(grpc::ServerContext* context,
//                                          const google::protobuf::Empty*
//                                          request, palm::pi::v1::Ether*
//                                          response) {
//   // TODO
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::SetEther(grpc::ServerContext* context,
//                                          const palm::pi::v1::Ether* request,
//                                          google::protobuf::Empty* response) {
//   // TODO
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::GetWifi(grpc::ServerContext* context,
//                                         const google::protobuf::Empty*
//                                         request, palm::pi::v1::Wifi*
//                                         response) {
//   // TODO
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::SetWifi(grpc::ServerContext* context,
//                                         const palm::pi::v1::Wifi* request,
//                                         google::protobuf::Empty* response) {
//   // TODO
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::Ntp(grpc::ServerContext* context,
//                                     const google::protobuf::Empty* request,
//                                     palm::pi::v1::LogsResponse* response) {
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::Dns(grpc::ServerContext* context,
//                                     const google::protobuf::Empty* request,
//                                     palm::pi::v1::LogsResponse* response) {
//   // TODO
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::GetVpn(grpc::ServerContext* context,
//                                        const google::protobuf::Empty*
//                                        request, palm::pi::v1::VpnProfile*
//                                        response) {
//   // TODO
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::SetVpn(grpc::ServerContext* context,
//                                        const palm::pi::v1::VpnProfile*
//                                        request, google::protobuf::Empty*
//                                        response) {
//   // TODO
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::GetNtp(grpc::ServerContext* context,
//                                        const google::protobuf::Empty*
//                                        request, palm::pi::v1::NtpProfile*
//                                        response) {
//   // TODO
//   return grpc::Status::OK;
// }
// grpc::Status palm::pi::Service::SetNtp(grpc::ServerContext* context,
//                                        const palm::pi::v1::NtpProfile*
//                                        request, google::protobuf::Empty*
//                                        response) {
//   // TODO
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::Reboot(grpc::ServerContext* context,
//                                        const google::protobuf::Empty*
//                                        request, google::protobuf::Empty*
//                                        response) {
//   // TODO check user
//   palm::reboot();
//   return grpc::Status::OK;
// }

// grpc::Status palm::pi::Service::Reset(grpc::ServerContext* context,
//                                       const google::protobuf::Empty* request,
//                                       google::protobuf::Empty* response) {
//   // TODO check user
//   std::filesystem::remove_all(ROOT);
//   palm::reboot();
//   return grpc::Status::OK;
// }
