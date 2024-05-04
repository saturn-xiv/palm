namespace cpp daisy.v1
namespace java com.github.saturn_xiv.palm.plugins.daisy.v1

struct Address {
    1:string name,
    2:string email,
}

struct Body {
    1:string text,
    2:bool html,
}

struct Attachment {
    1:string title,
    2:string content_type,
    8:bool inline,
    9:binary body,
}

struct EmailSendTask {  
  1:Address to,
  2:set<Address> cc;
  3:set<Address> bcc;

  11:string subject;
  12:Body body;
  19:set<Attachment> attachments;
}
