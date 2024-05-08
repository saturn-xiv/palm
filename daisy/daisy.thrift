namespace cpp daisy.v1
namespace java com.github.saturn_xiv.palm.plugins.daisy.v1

struct Address {
    1:required string name,
    2:required string email,
}

struct Body {
    1:required string text,
    2:required bool html,
}

struct Attachment {
    1:required string title,
    2:required string content_type,
    8:required bool inline_,
    9:required binary body,
}

struct EmailSendTask {  
  1:required Address to,
  2:required set<Address> cc;
  3:required set<Address> bcc;

  11:required string subject;
  12:required Body body;
  19:required set<Attachment> attachments;
}
