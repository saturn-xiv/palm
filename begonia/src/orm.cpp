#include "palm/orm.hpp"

std::pair<int, int> palm::paginate(int total, int index, int size) {
  const int MIN_SIZE = 10, MAX_SIZE = (1 << 12);

  if (size < MIN_SIZE) {
    size = MIN_SIZE;
  }
  if (size > MAX_SIZE) {
    size = MAX_SIZE;
  }
  if (index < 1) {
    index = 1;
  }
  if (total <= size) {
    return {1, size};
  }
  if (total < index * size) {
    if (total % size == 0) {
      return {total / size, size};
    }
    return {(total / size) + 1, size};
  }
  return {index, size};
}
