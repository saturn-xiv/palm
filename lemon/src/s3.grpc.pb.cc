// Generated by the gRPC C++ plugin.
// If you make any local change, they will be lost.
// source: s3.proto

#include "s3.pb.h"
#include "s3.grpc.pb.h"

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
namespace s3 {
namespace v1 {

static const char* S3_method_names[] = {
  "/palm.s3.v1.S3/CreateBucket",
  "/palm.s3.v1.S3/UploadObject",
  "/palm.s3.v1.S3/ObjectPermanentUrl",
  "/palm.s3.v1.S3/ObjectPresignedUrl",
  "/palm.s3.v1.S3/RemoveObject",
};

std::unique_ptr< S3::Stub> S3::NewStub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options) {
  (void)options;
  std::unique_ptr< S3::Stub> stub(new S3::Stub(channel, options));
  return stub;
}

S3::Stub::Stub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options)
  : channel_(channel), rpcmethod_CreateBucket_(S3_method_names[0], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  , rpcmethod_UploadObject_(S3_method_names[1], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  , rpcmethod_ObjectPermanentUrl_(S3_method_names[2], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  , rpcmethod_ObjectPresignedUrl_(S3_method_names[3], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  , rpcmethod_RemoveObject_(S3_method_names[4], options.suffix_for_stats(),::grpc::internal::RpcMethod::NORMAL_RPC, channel)
  {}

::grpc::Status S3::Stub::CreateBucket(::grpc::ClientContext* context, const ::palm::s3::v1::CreateBucketRequest& request, ::palm::s3::v1::CreateBucketResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::palm::s3::v1::CreateBucketRequest, ::palm::s3::v1::CreateBucketResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_CreateBucket_, context, request, response);
}

void S3::Stub::async::CreateBucket(::grpc::ClientContext* context, const ::palm::s3::v1::CreateBucketRequest* request, ::palm::s3::v1::CreateBucketResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::palm::s3::v1::CreateBucketRequest, ::palm::s3::v1::CreateBucketResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_CreateBucket_, context, request, response, std::move(f));
}

void S3::Stub::async::CreateBucket(::grpc::ClientContext* context, const ::palm::s3::v1::CreateBucketRequest* request, ::palm::s3::v1::CreateBucketResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_CreateBucket_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::CreateBucketResponse>* S3::Stub::PrepareAsyncCreateBucketRaw(::grpc::ClientContext* context, const ::palm::s3::v1::CreateBucketRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::palm::s3::v1::CreateBucketResponse, ::palm::s3::v1::CreateBucketRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_CreateBucket_, context, request);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::CreateBucketResponse>* S3::Stub::AsyncCreateBucketRaw(::grpc::ClientContext* context, const ::palm::s3::v1::CreateBucketRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncCreateBucketRaw(context, request, cq);
  result->StartCall();
  return result;
}

::grpc::Status S3::Stub::UploadObject(::grpc::ClientContext* context, const ::palm::s3::v1::UploadObjectRequest& request, ::palm::s3::v1::UploadObjectResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::palm::s3::v1::UploadObjectRequest, ::palm::s3::v1::UploadObjectResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_UploadObject_, context, request, response);
}

void S3::Stub::async::UploadObject(::grpc::ClientContext* context, const ::palm::s3::v1::UploadObjectRequest* request, ::palm::s3::v1::UploadObjectResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::palm::s3::v1::UploadObjectRequest, ::palm::s3::v1::UploadObjectResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_UploadObject_, context, request, response, std::move(f));
}

void S3::Stub::async::UploadObject(::grpc::ClientContext* context, const ::palm::s3::v1::UploadObjectRequest* request, ::palm::s3::v1::UploadObjectResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_UploadObject_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::UploadObjectResponse>* S3::Stub::PrepareAsyncUploadObjectRaw(::grpc::ClientContext* context, const ::palm::s3::v1::UploadObjectRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::palm::s3::v1::UploadObjectResponse, ::palm::s3::v1::UploadObjectRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_UploadObject_, context, request);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::UploadObjectResponse>* S3::Stub::AsyncUploadObjectRaw(::grpc::ClientContext* context, const ::palm::s3::v1::UploadObjectRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncUploadObjectRaw(context, request, cq);
  result->StartCall();
  return result;
}

::grpc::Status S3::Stub::ObjectPermanentUrl(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPermanentUrlRequest& request, ::palm::s3::v1::UrlResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::palm::s3::v1::ObjectPermanentUrlRequest, ::palm::s3::v1::UrlResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_ObjectPermanentUrl_, context, request, response);
}

void S3::Stub::async::ObjectPermanentUrl(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPermanentUrlRequest* request, ::palm::s3::v1::UrlResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::palm::s3::v1::ObjectPermanentUrlRequest, ::palm::s3::v1::UrlResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_ObjectPermanentUrl_, context, request, response, std::move(f));
}

void S3::Stub::async::ObjectPermanentUrl(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPermanentUrlRequest* request, ::palm::s3::v1::UrlResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_ObjectPermanentUrl_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::UrlResponse>* S3::Stub::PrepareAsyncObjectPermanentUrlRaw(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPermanentUrlRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::palm::s3::v1::UrlResponse, ::palm::s3::v1::ObjectPermanentUrlRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_ObjectPermanentUrl_, context, request);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::UrlResponse>* S3::Stub::AsyncObjectPermanentUrlRaw(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPermanentUrlRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncObjectPermanentUrlRaw(context, request, cq);
  result->StartCall();
  return result;
}

::grpc::Status S3::Stub::ObjectPresignedUrl(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPresignedUrlRequest& request, ::palm::s3::v1::UrlResponse* response) {
  return ::grpc::internal::BlockingUnaryCall< ::palm::s3::v1::ObjectPresignedUrlRequest, ::palm::s3::v1::UrlResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_ObjectPresignedUrl_, context, request, response);
}

void S3::Stub::async::ObjectPresignedUrl(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPresignedUrlRequest* request, ::palm::s3::v1::UrlResponse* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::palm::s3::v1::ObjectPresignedUrlRequest, ::palm::s3::v1::UrlResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_ObjectPresignedUrl_, context, request, response, std::move(f));
}

void S3::Stub::async::ObjectPresignedUrl(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPresignedUrlRequest* request, ::palm::s3::v1::UrlResponse* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_ObjectPresignedUrl_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::UrlResponse>* S3::Stub::PrepareAsyncObjectPresignedUrlRaw(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPresignedUrlRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::palm::s3::v1::UrlResponse, ::palm::s3::v1::ObjectPresignedUrlRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_ObjectPresignedUrl_, context, request);
}

::grpc::ClientAsyncResponseReader< ::palm::s3::v1::UrlResponse>* S3::Stub::AsyncObjectPresignedUrlRaw(::grpc::ClientContext* context, const ::palm::s3::v1::ObjectPresignedUrlRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncObjectPresignedUrlRaw(context, request, cq);
  result->StartCall();
  return result;
}

::grpc::Status S3::Stub::RemoveObject(::grpc::ClientContext* context, const ::palm::s3::v1::RemoveObjectRequest& request, ::google::protobuf::Empty* response) {
  return ::grpc::internal::BlockingUnaryCall< ::palm::s3::v1::RemoveObjectRequest, ::google::protobuf::Empty, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), rpcmethod_RemoveObject_, context, request, response);
}

void S3::Stub::async::RemoveObject(::grpc::ClientContext* context, const ::palm::s3::v1::RemoveObjectRequest* request, ::google::protobuf::Empty* response, std::function<void(::grpc::Status)> f) {
  ::grpc::internal::CallbackUnaryCall< ::palm::s3::v1::RemoveObjectRequest, ::google::protobuf::Empty, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_RemoveObject_, context, request, response, std::move(f));
}

void S3::Stub::async::RemoveObject(::grpc::ClientContext* context, const ::palm::s3::v1::RemoveObjectRequest* request, ::google::protobuf::Empty* response, ::grpc::ClientUnaryReactor* reactor) {
  ::grpc::internal::ClientCallbackUnaryFactory::Create< ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(stub_->channel_.get(), stub_->rpcmethod_RemoveObject_, context, request, response, reactor);
}

::grpc::ClientAsyncResponseReader< ::google::protobuf::Empty>* S3::Stub::PrepareAsyncRemoveObjectRaw(::grpc::ClientContext* context, const ::palm::s3::v1::RemoveObjectRequest& request, ::grpc::CompletionQueue* cq) {
  return ::grpc::internal::ClientAsyncResponseReaderHelper::Create< ::google::protobuf::Empty, ::palm::s3::v1::RemoveObjectRequest, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(channel_.get(), cq, rpcmethod_RemoveObject_, context, request);
}

::grpc::ClientAsyncResponseReader< ::google::protobuf::Empty>* S3::Stub::AsyncRemoveObjectRaw(::grpc::ClientContext* context, const ::palm::s3::v1::RemoveObjectRequest& request, ::grpc::CompletionQueue* cq) {
  auto* result =
    this->PrepareAsyncRemoveObjectRaw(context, request, cq);
  result->StartCall();
  return result;
}

S3::Service::Service() {
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      S3_method_names[0],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< S3::Service, ::palm::s3::v1::CreateBucketRequest, ::palm::s3::v1::CreateBucketResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](S3::Service* service,
             ::grpc::ServerContext* ctx,
             const ::palm::s3::v1::CreateBucketRequest* req,
             ::palm::s3::v1::CreateBucketResponse* resp) {
               return service->CreateBucket(ctx, req, resp);
             }, this)));
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      S3_method_names[1],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< S3::Service, ::palm::s3::v1::UploadObjectRequest, ::palm::s3::v1::UploadObjectResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](S3::Service* service,
             ::grpc::ServerContext* ctx,
             const ::palm::s3::v1::UploadObjectRequest* req,
             ::palm::s3::v1::UploadObjectResponse* resp) {
               return service->UploadObject(ctx, req, resp);
             }, this)));
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      S3_method_names[2],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< S3::Service, ::palm::s3::v1::ObjectPermanentUrlRequest, ::palm::s3::v1::UrlResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](S3::Service* service,
             ::grpc::ServerContext* ctx,
             const ::palm::s3::v1::ObjectPermanentUrlRequest* req,
             ::palm::s3::v1::UrlResponse* resp) {
               return service->ObjectPermanentUrl(ctx, req, resp);
             }, this)));
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      S3_method_names[3],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< S3::Service, ::palm::s3::v1::ObjectPresignedUrlRequest, ::palm::s3::v1::UrlResponse, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](S3::Service* service,
             ::grpc::ServerContext* ctx,
             const ::palm::s3::v1::ObjectPresignedUrlRequest* req,
             ::palm::s3::v1::UrlResponse* resp) {
               return service->ObjectPresignedUrl(ctx, req, resp);
             }, this)));
  AddMethod(new ::grpc::internal::RpcServiceMethod(
      S3_method_names[4],
      ::grpc::internal::RpcMethod::NORMAL_RPC,
      new ::grpc::internal::RpcMethodHandler< S3::Service, ::palm::s3::v1::RemoveObjectRequest, ::google::protobuf::Empty, ::grpc::protobuf::MessageLite, ::grpc::protobuf::MessageLite>(
          [](S3::Service* service,
             ::grpc::ServerContext* ctx,
             const ::palm::s3::v1::RemoveObjectRequest* req,
             ::google::protobuf::Empty* resp) {
               return service->RemoveObject(ctx, req, resp);
             }, this)));
}

S3::Service::~Service() {
}

::grpc::Status S3::Service::CreateBucket(::grpc::ServerContext* context, const ::palm::s3::v1::CreateBucketRequest* request, ::palm::s3::v1::CreateBucketResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}

::grpc::Status S3::Service::UploadObject(::grpc::ServerContext* context, const ::palm::s3::v1::UploadObjectRequest* request, ::palm::s3::v1::UploadObjectResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}

::grpc::Status S3::Service::ObjectPermanentUrl(::grpc::ServerContext* context, const ::palm::s3::v1::ObjectPermanentUrlRequest* request, ::palm::s3::v1::UrlResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}

::grpc::Status S3::Service::ObjectPresignedUrl(::grpc::ServerContext* context, const ::palm::s3::v1::ObjectPresignedUrlRequest* request, ::palm::s3::v1::UrlResponse* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}

::grpc::Status S3::Service::RemoveObject(::grpc::ServerContext* context, const ::palm::s3::v1::RemoveObjectRequest* request, ::google::protobuf::Empty* response) {
  (void) context;
  (void) request;
  (void) response;
  return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
}


}  // namespace palm
}  // namespace s3
}  // namespace v1

