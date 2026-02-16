#pragma once

#include "email.grpc.pb.h"
#include "portal.grpc.pb.h"
#include "rbac.grpc.pb.h"
#include "s3.grpc.pb.h"
#include "sms.grpc.pb.h"
#include "tex.grpc.pb.h"
#include "wechat-pay.grpc.pb.h"

namespace tulip {
void init(bool debug = false);
}
