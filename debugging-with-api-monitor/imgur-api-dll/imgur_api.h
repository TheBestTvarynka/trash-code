// generated with the following command:
// cbindgen --crate imgur-api-dll --output imgur_api.h

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

using FfiStr = const char*;

struct FfiCommentData {
  uint64_t id;
  FfiStr image_id;
  FfiStr comment;
  FfiStr author;
  uint64_t author_id;
  bool on_album;
  FfiStr album_cover;
  uint32_t ups;
  uint32_t downs;
  uint32_t points;
  uint32_t datetime;
  uint64_t parent_id;
  bool deleted;
  bool is_voted;
  bool vote;
  FfiStr platform;
  bool has_admin_badge;
  const uint64_t *children;
  uint32_t children_len;
};

struct FiiComment {
  FfiCommentData data;
  uint32_t status;
  bool success;
};

extern "C" {

void *ImgurInitClient(const char *client_id, const char *client_secret);

uint32_t ImgurGetComment(void *context, unsigned long long comment_id, FiiComment **comment);

} // extern "C"
