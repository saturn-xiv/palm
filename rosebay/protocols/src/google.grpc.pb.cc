// Generated by the gRPC C++ plugin.
// If you make any local change, they will be lost.
// source: google.proto

#include "google.pb.h"
#include "google.grpc.pb.h"

#include <functional>
#include <grpcpp/support/async_stream.h>
#include <grpcpp/support/async_unary_call.h>
#include <grpcpp/impl/channel_interface.h>
#include <grpcpp/impl/client_unary_call.h>
#include <grpcpp/support/client_callback.h>
#include <grpcpp/support/message_allocator.h>
#include <grpcpp/support/method_handler.h>
#include <grpcpp/impl/rpc_service_method.h>
#include <grpcpp/support/server_callback.h>
#include <grpcpp/impl/server_callback_handlers.h>
#include <grpcpp/server_context.h>
#include <grpcpp/impl/service_type.h>
#include <grpcpp/support/sync_stream.h>
namespace palm {
namespace google {
namespace v1 {

static const char* Oauth2_method_names[] = {
  "/palm.google.v1.Oauth2/AuthCodeURL",
  "/palm.google.v1.Oauth2/SignIn",
};

std::unique_ptr< Oauth2::Stub> Oauth2::NewStub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options) {
  (void)options;
  std::unique_ptr< Oauth2::Stub> stub(new Oauth2::Stub(channel, options));
  return stub;
}

Oauth2::Stub::Stub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options)
  : channel_(channel), rpcmethod_AuthCodeURL_(Oauth2_method_names[0], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  , rpcmethod_SignIn_(Oauth2_method_names[1], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  {}

::grpc::Status Oauth2::Stub::AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_AuthCodeURL_, context, request, response);
}

void Oauth2::Stub::async::AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_AuthCodeURL_, context, request, response, std::move(f));
}

void Oauth2::Stub::async::AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_AuthCodeURL_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>* Oauth2::Stub::PrepareAsyncAuthCodeURLRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::palm::google::v1::Oauth2AuthCodeURLResponse, ::palm::google::v1::Oauth2AuthCodeURLRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_AuthCodeURL_, context, request);
}

::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>* Oauth2::Stub::AsyncAuthCodeURLRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncAuthCodeURLRaw(context, request, cq);
  result->StartCall();
  return result;
}

::grpc::Status Oauth2::Stub::SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::palm::google::v1::Oauth2SignInResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_SignIn_, context, request, response);
}

void Oauth2::Stub::async::SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_SignIn_, context, request, response, std::move(f));
}

void Oauth2::Stub::async::SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_SignIn_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>* Oauth2::Stub::PrepareAsyncSignInRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::palm::google::v1::Oauth2SignInResponse, ::palm::google::v1::Oauth2SignInRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_SignIn_, context, request);
}

::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>* Oauth2::Stub::AsyncSignInRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncSignInRaw(context, request, cq);
  result->StartCall();
  return result;
}

Oauth2::Service::Service() {
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      Oauth2_method_names[0],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< Oauth2::Service, ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](Oauth2::Service* service,
             ::grpc::ServerContext* ctx,
             const ::palm::google::v1::Oauth2AuthCodeURLRequest* req,
             ::palm::google::v1::Oauth2AuthCodeURLResponse* resp) {
               return service->AuthCodeURL(ctx, req, resp);
             }, this)));
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      Oauth2_method_names[1],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< Oauth2::Service, ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](Oauth2::Service* service,
             ::grpc::ServerContext* ctx,
             const ::palm::google::v1::Oauth2SignInRequest* req,
             ::palm::google::v1::Oauth2SignInResponse* resp) {
               return service->SignIn(ctx, req, resp);
             }, this)));
}

Oauth2::Service::~Service() {
}

::grpc::Status Oauth2::Service::AuthCodeURL(::grpc::ServerContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}

::grpc::Status Oauth2::Service::SignIn(::grpc::ServerContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}


}  // namespace palm
}  // namespace google
}  // namespace v1

