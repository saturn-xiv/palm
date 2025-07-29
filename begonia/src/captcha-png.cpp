#include "palm/captcha.hpp"

#include <png.h>
#include <spdlog/spdlog.h>

static void png_write_callback(png_structp ptr, png_bytep buf, png_size_t len) {
  std::vector<uint8_t> *p = (std::vector<uint8_t> *)png_get_io_ptr(ptr);
  p->insert(p->end(), buf, buf + len);
}

struct PngDestructor {
  png_struct *p;
  PngDestructor(png_struct *p) : p(p) {}
  ~PngDestructor() {
    if (p != nullptr) {
      png_destroy_write_struct(&p, NULL);
    }
  }
};

static inline void write_png_to_memory(size_t width, size_t height,
                                       const uint8_t *data,
                                       std::vector<uint8_t> *out) {
  spdlog::debug("generate a png buffer({}, {})", width, height);
  out->clear();
  png_structp ptr =
      png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
  if (ptr == nullptr) {
    spdlog::error("png_create_write_struct failed");
    return;
  }

  PngDestructor destructor(ptr);
  png_infop ifp = png_create_info_struct(ptr);
  if (ifp == nullptr) {
    spdlog::error("png_create_info_struct failed");
    return;
  }
  if (0 != setjmp(png_jmpbuf(ptr))) {
    spdlog::error("setjmp failed");
    return;
  }

  png_set_IHDR(ptr, ifp, width, height, 8, PNG_COLOR_TYPE_RGBA,
               PNG_INTERLACE_NONE, PNG_COMPRESSION_TYPE_DEFAULT,
               PNG_FILTER_TYPE_DEFAULT);
  // png_set_compression_level(p, 1);
  std::vector<uint8_t *> rows(height);
  for (size_t y = 0; y < height; ++y) {
    rows[y] = (uint8_t *)data + y * width * 4;
  }
  png_set_rows(ptr, ifp, &rows[0]);
  png_set_write_fn(ptr, out, png_write_callback, NULL);
  png_write_png(ptr, ifp, PNG_TRANSFORM_IDENTITY, NULL);
}

std::vector<uint8_t> palm::captcha::png(const std::string &str, uint8_t size) {
  const uint8_t MIN_SIZE = 16;
  const uint8_t PAD = 6;
  if (size < MIN_SIZE) {
    size = MIN_SIZE;
  }
  const auto len = str.length();

  const size_t width = (size + PAD) * len + PAD;
  const size_t height = size + PAD;
}
