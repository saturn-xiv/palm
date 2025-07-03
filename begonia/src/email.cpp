#include "palm/email.hpp"

#include <list>

#include <spdlog/spdlog.h>
#include <mailio/message.hpp>
#include <mailio/smtp.hpp>

void palm::email::Smtp::send(const Account& to, const std::vector<Account> cc,
                             const std::vector<Account> bcc,
                             const std::string& subject, const Body& body,
                             const std::vector<Attachment> attachments) const {
  mailio::message msg;
  msg.from(mailio::mail_address(this->_user.name, this->_user.email));
  msg.add_recipient(mailio::mail_address(to.name, to.email));
  for (const auto& it : cc) {
    msg.add_cc_recipient(mailio::mail_address(it.name, it.email));
  }
  for (const auto& it : bcc) {
    msg.add_bcc_recipient(mailio::mail_address(it.name, it.email));
  }
  msg.subject(subject);
  msg.content_type(mailio::message::media_type_t::MULTIPART, "related");

  {
    mailio::mime content;
    content.content_type(mailio::message::media_type_t::TEXT,
                         body.html ? "html" : "plain", "utf-8");
    content.content_transfer_encoding(
        mailio::mime::content_transfer_encoding_t::BIT_8);
    content.content(body.content);

    msg.add_part(content);
  }

  for (const auto& it : attachments) {
    mailio::mime att;
    if (it.content_type.first == "image") {
      att.content_type(mailio::message::media_type_t::IMAGE,
                       it.content_type.second);
      att.content_disposition(mailio::mime::content_disposition_t::INLINE);
    } else if (it.content_type.first == "audio") {
      att.content_type(mailio::message::media_type_t::AUDIO,
                       it.content_type.second);
      att.content_disposition(mailio::mime::content_disposition_t::INLINE);
    } else if (it.content_type.first == "video") {
      att.content_type(mailio::message::media_type_t::VIDEO,
                       it.content_type.second);
      att.content_disposition(mailio::mime::content_disposition_t::INLINE);
    } else if (it.content_type.first == "text") {
      att.content_type(mailio::message::media_type_t::TEXT,
                       it.content_type.second);
      att.content_disposition(mailio::mime::content_disposition_t::INLINE);
    } else {
      spdlog::warn("undetected content-type {}/{}", it.content_type.first,
                   it.content_type.second);
      att.content_type(mailio::message::media_type_t::APPLICATION,
                       "octet-stream");
      att.content_disposition(mailio::mime::content_disposition_t::ATTACHMENT);
    }

    att.content_transfer_encoding(
        mailio::mime::content_transfer_encoding_t::BASE_64);

    {
      std::string body(it.body.begin(), it.body.end());
      att.content(body);
    }
    att.name(it.name);

    msg.add_part(att);
  }
  spdlog::info("send email to {}/{}: {}", to.name, to.email, subject);
  mailio::smtps con(this->_host, this->_port);

  con.authenticate(this->_user.email, this->_password,
                   mailio::smtps::auth_method_t::LOGIN);
  con.submit(msg);
}
