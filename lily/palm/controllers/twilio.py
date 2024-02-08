import logging

from flask import make_response, request

from . import app

# https://www.twilio.com/docs/messaging/tutorials/how-to-receive-and-reply/python


@app.route("/twilio/callback/incoming-text-message", methods=['POST'])
def sms_reply():
    body = request.values.get('Body', None)
    logging.info("receive sms message %s", body)
    msg = MessagingResponse()
    msg.message("Succeed!")

    res = make_response(str(msg))
    res.headers['Content-type'] = 'application/xml; charset=utf-8'
    return res

# https://www.twilio.com/docs/messaging/guides/track-outbound-message-status


@app.route("/twilio/callback/message-status", methods=['POST'])
def incoming_sms():
    message_sid = request.values.get('MessageSid', None)
    message_status = request.values.get('MessageStatus', None)
    logging.info('SID: %s, Status: %s', message_sid, message_status)

    res = make_response("", 204)
    res.headers['Content-type'] = 'text/plain; charset=utf-8'
    return res
