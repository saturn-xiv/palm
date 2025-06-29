#define BOOST_TEST_MODULE email
#include <boost/test/included/unit_test.hpp>

#include "basil/email.hpp"
#include "basil/utils.hpp"

#include <cstdlib>

BOOST_AUTO_TEST_CASE(smtp) {
  const std::string host = "smtp.gmail.com";
  const uint16_t port = 465;
  const std::string password = std::getenv("SMTP_FROM_PASSWORD");
  //   https://developers.google.com/workspace/gmail/imap/imap-smtp
  basil::email::Account from;
  ;
  {
    from.name = std::getenv("SMTP_FROM_NAME");
    from.email = std::getenv("SMTP_FROM_EMAIL");
  }
  basil::email::Account to;
  {
    to.name = std::getenv("SMTP_TO_NAME");
    to.email = std::getenv("SMTP_TO_EMAIL");
  }

  basil::email::Smtp con(host, port, from, password);

  {
    const std::string subject = "local test";
    basil::email::Body body;
    body.content = "<h2>Hi,</h2><br/>";
    body.html = true;

    std::vector<basil::email::Attachment> attachments;
    {
      basil::email::Attachment it;
      it.content_type = {"text", "plain"};
      {
        std::filesystem::path file("LICENSE");
        basil::load(file, it.body);
      }
      it.name = "license.txt";
      attachments.push_back(it);
    }

    con.send(to, {}, {}, subject, body, attachments);
  }
}
