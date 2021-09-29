// #include "palm/ops-email.hpp"

// #include <unistd.h>
// #include <algorithm>

// std::string ssh512_sum(const std::string& plain, const std::string& salt) {
//   auto password = crypt(
//       plain.c_str(), (palm::ops::email::ssha512::SALT_HEADER +
//       salt).c_str());
//   return palm::ops::email::ssha512::PASSWORD_HEADER + password;
// }

// std::string palm::ops::email::ssha512::sum(const std::string& plain) {
//   auto rand_char = []() -> char {
//     const char charset[] =
//         "0123456789"
//         "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
//         "abcdefghijklmnopqrstuvwxyz";
//     const size_t max_index = (sizeof(charset) - 1);
//     return charset[rand() % max_index];
//   };
//   std::string salt(SALT_LEN, 0);
//   std::generate_n(salt.begin(), SALT_LEN, rand_char);

//   return ssh512_sum(plain, salt);
// }
// bool palm::ops::email::ssha512::verify(const std::string& password,
//                                        const std::string& plain) {
//   const std::string salt =
//       password.substr(PASSWORD_HEADER.size() + SALT_HEADER.size(), SALT_LEN);
//   return password == ssh512_sum(plain, salt);
// }
