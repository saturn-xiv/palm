#include <catch2/catch_test_macros.hpp>

#include "palm/email.hpp"
#include "palm/utils.hpp"

#include <cstdlib>

TEST_CASE("send email by google", "[smtp]") {
  SECTION("by google") {
    const std::string host = "smtp.gmail.com";
    const uint16_t port = 465;
    const std::string password = std::getenv("SMTP_FROM_PASSWORD");
    //   https://developers.google.com/workspace/gmail/imap/imap-smtp
    palm::email::Account from;

    {
      from.name = std::getenv("SMTP_FROM_NAME");
      from.email = std::getenv("SMTP_FROM_EMAIL");
    }
    palm::email::Account to;
    {
      to.name = std::getenv("SMTP_TO_NAME");
      to.email = std::getenv("SMTP_TO_EMAIL");
    }

    palm::email::Smtp con(host, port, from, password);

    {
      const std::string subject = "local test";
      palm::email::Body body;
      body.content = "<h2>Hi,</h2><br/>";
      body.html = true;

      std::vector<palm::email::Attachment> attachments;
      {
        palm::email::Attachment it;
        it.content_type = {"text", "plain"};
        {
          std::filesystem::path file("LICENSE");
          palm::load(file, it.body);
        }
        it.name = "license.txt";
        attachments.push_back(it);
      }

      con.send(to, {}, {}, subject, body, attachments);
    }
  }
}
