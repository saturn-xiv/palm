#include "palm/captcha.hpp"
#include "palm/crypto.hpp"

#include <filesystem>

#include <png.h>
#include <spdlog/spdlog.h>

#include <ft2build.h>
#include FT_FREETYPE_H

// static void png_write_callback(png_structp ptr, png_bytep buf, png_size_t
// len) {
//   std::vector<uint8_t> *p = (std::vector<uint8_t> *)png_get_io_ptr(ptr);
//   p->insert(p->end(), buf, buf + len);
// }

// struct PngDestructor {
//   png_struct *p;
//   PngDestructor(png_struct *p) : p(p) {}
//   ~PngDestructor() {
//     if (p != nullptr) {
//       png_destroy_write_struct(&p, NULL);
//     }
//   }
// };

// static inline void write_png_to_memory(size_t width, size_t height,
//                                        const uint8_t *data,
//                                        std::vector<uint8_t> *out) {
//   spdlog::debug("generate a png buffer({}, {})", width, height);
//   out->clear();
//   png_structp ptr =
//       png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
//   if (ptr == nullptr) {
//     spdlog::error("png_create_write_struct failed");
//     return;
//   }

//   PngDestructor destructor(ptr);
//   png_infop ifp = png_create_info_struct(ptr);
//   if (ifp == nullptr) {
//     spdlog::error("png_create_info_struct failed");
//     return;
//   }
//   if (0 != setjmp(png_jmpbuf(ptr))) {
//     spdlog::error("setjmp failed");
//     return;
//   }

//   png_set_IHDR(ptr, ifp, width, height, 8, PNG_COLOR_TYPE_RGBA,
//                PNG_INTERLACE_NONE, PNG_COMPRESSION_TYPE_DEFAULT,
//                PNG_FILTER_TYPE_DEFAULT);
//   // png_set_compression_level(p, 1);
//   std::vector<uint8_t *> rows(height);
//   for (size_t y = 0; y < height; ++y) {
//     rows[y] = (uint8_t *)data + y * width * 4;
//   }
//   png_set_rows(ptr, ifp, &rows[0]);
//   png_set_write_fn(ptr, out, png_write_callback, NULL);
//   png_write_png(ptr, ifp, PNG_TRANSFORM_IDENTITY, NULL);
// }

typedef struct {
  png_bytep buffer;
  png_size_t size;
} PngMemoryWriterState;

static void write_data_memory(png_structp png_ptr, png_bytep data,
                              png_size_t length) {
  PngMemoryWriterState *p = (PngMemoryWriterState *)png_get_io_ptr(png_ptr);
  png_size_t new_size = p->size + length;

  if (p->buffer) {
    p->buffer = (png_bytep)realloc(p->buffer, new_size);
  } else {
    p->buffer = (png_bytep)malloc(new_size);
  }

  if (!p->buffer) {
    spdlog::error("failed to allocate memory for PNG buffer");
    return;
  }

  memcpy(p->buffer + p->size, data, length);
  p->size = new_size;
}
static void flush_data_memory(png_structp png_ptr) {
  spdlog::debug("flush png data memory");
}

// https://mrandri19.github.io/2019/07/18/modern-text-rendering-linux-ep1.html
// https://freetype.org/freetype2/docs/tutorial/step1.html#section-2
static void write_captcha(png_structp png_ptr, png_bytep row,
                          const std::string &str, size_t font_size) {
  FT_Library ft;
  if (FT_Init_FreeType(&ft) != 0) {
    spdlog::error("failed to initialize FreeType");
    return;
  }
  FT_Face face;
  {
    const char *DEJAVU_FONT = "/usr/share/fonts/TTF/DejaVuSansMono.ttf";
    if (std::filesystem::exists(DEJAVU_FONT)) {
      if (FT_New_Face(ft, DEJAVU_FONT, 0, &face) != 0) {
        spdlog::error("failed to load face(dejavu font)");
        return;
      }
    }
  }

  if (FT_Set_Pixel_Sizes(face, 0, font_size) != 0) {
    spdlog::error("failed to set pixel size({})", font_size);
    return;
  }
  for (const char &c : str) {
    FT_UInt glyph_index = FT_Get_Char_Index(face, c);
    if (FT_Load_Glyph(face, glyph_index, FT_LOAD_DEFAULT) != 0) {
      spdlog::error("failed to load glyph for char '{}'", c);
      return;
    }
    if (FT_Render_Glyph(face->glyph, FT_RENDER_MODE_NORMAL) != 0) {
      spdlog::error("failed to render the glyph");
      return;
    }
    for (size_t i = 0; i < face->glyph->bitmap.rows; i++) {
      for (size_t j = 0; j < face->glyph->bitmap.width; j++) {
        unsigned char pixel_brightness =
            face->glyph->bitmap.buffer[i * face->glyph->bitmap.pitch + j];
        if (pixel_brightness > 84) {
          // TODO drow pixel
        } else {
          // TODO random
        }
      }
    }
  }
}

// http://www.labbookpages.co.uk/software/imgProc/libPNG.html
static void create_png(const std::string &str, size_t width, size_t height,
                       std::vector<uint8_t> &out) {
  png_structp png_ptr = NULL;
  png_infop info_ptr = NULL;
  png_bytep row = NULL;

  PngMemoryWriterState writer_state = {NULL, 0};

  png_ptr = png_create_write_struct(PNG_LIBPNG_VER_STRING, NULL, NULL, NULL);
  if (png_ptr == NULL) {
    spdlog::error("could not allocate png write struct");
    goto finalize;
  }
  info_ptr = png_create_info_struct(png_ptr);
  if (info_ptr == NULL) {
    spdlog::error("could not allocate png info struct");
    goto finalize;
  }
  // setup Exception handling
  if (setjmp(png_jmpbuf(png_ptr))) {
    spdlog::error("error during png creation");
    goto finalize;
  }

  png_set_write_fn(png_ptr, &writer_state, write_data_memory,
                   flush_data_memory);

  // write header (8 bit colour depth)
  png_set_IHDR(png_ptr, info_ptr, width, height, 8, PNG_COLOR_TYPE_RGB,
               PNG_INTERLACE_NONE, PNG_COMPRESSION_TYPE_BASE,
               PNG_FILTER_TYPE_BASE);
  // set title
  {
    png_text title;
    title.compression = PNG_TEXT_COMPRESSION_NONE;
    title.key = "Title";
    title.text = "Captcha";
    png_set_text(png_ptr, info_ptr, &title, 1);
  }
  png_write_info(png_ptr, info_ptr);

  // allocate memory for one row (3 bytes per pixel - RGB)
  // row = (png_bytep)malloc(3 * width * sizeof(png_byte));
  // TODO
  row = (png_bytep)malloc(png_get_rowbytes(png_ptr, info_ptr));
  {
    for (int y = 0; y < height; y++) {
      for (int x = 0; x < width; x++) {
        const auto [r, g, b] = palm::random::rgb();
        row[x * 3] = r;      // Red
        row[x * 3 + 1] = g;  // Green
        row[x * 3 + 2] = b;  // Blue
      }
      png_write_row(png_ptr, row);
    }
  }
  // end write
  png_write_end(png_ptr, NULL);

finalize:
  if (info_ptr != NULL) {
    png_free_data(png_ptr, info_ptr, PNG_FREE_ALL, -1);
  }
  if (png_ptr != NULL) {
    png_destroy_write_struct(&png_ptr, (png_infopp)NULL);
  }
  if (row != NULL) {
    free(row);
  }
  if (writer_state.buffer != NULL) {
    out.resize(writer_state.size);
    std::memcpy(out.data(), (void *)writer_state.buffer, writer_state.size);
    free(writer_state.buffer);
  }
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
  spdlog::debug("generate a png {}x{}", width, height);
  std::vector<uint8_t> buf;
  create_png(str, width, height, buf);
  return buf;
}
