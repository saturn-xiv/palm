// Generated by the gRPC C++ plugin.
// If you make any local change, they will be lost.
// source: morus.proto
#ifndef GRPC_morus_2eproto__INCLUDED
#define GRPC_morus_2eproto__INCLUDED

#include "morus.pb.h"

#include <functional>
#include <grpcpp/generic/async_generic_service.h>
#include <grpcpp/support/async_stream.h>
#include <grpcpp/support/async_unary_call.h>
#include <grpcpp/support/client_callback.h>
#include <grpcpp/client_context.h>
#include <grpcpp/completion_queue.h>
#include <grpcpp/support/message_allocator.h>
#include <grpcpp/support/method_handler.h>
#include <grpcpp/impl/proto_utils.h>
#include <grpcpp/impl/rpc_method.h>
#include <grpcpp/support/server_callback.h>
#include <grpcpp/impl/server_callback_handlers.h>
#include <grpcpp/server_context.h>
#include <grpcpp/impl/service_type.h>
#include <grpcpp/support/status.h>
#include <grpcpp/support/stub_options.h>
#include <grpcpp/support/sync_stream.h>

namespace palm {
namespace morus {
namespace v1 {

class Markdown final {
 public:
  static constexpr char const* service_full_name() {
    return "palm.morus.v1.Markdown";
  }
  class StubInterface {
   public:
    virtual ~StubInterface() {}
    virtual ::grpc::Status ToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::palm::morus::v1::MarkdownToHtmlResponse* response) = 0;
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::morus::v1::MarkdownToHtmlResponse>> AsyncToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::morus::v1::MarkdownToHtmlResponse>>(AsyncToHtmlRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::morus::v1::MarkdownToHtmlResponse>> PrepareAsyncToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::morus::v1::MarkdownToHtmlResponse>>(PrepareAsyncToHtmlRaw(context, request, cq));
    }
    class async_interface {
     public:
      virtual ~async_interface() {}
      virtual void ToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest* request, ::palm::morus::v1::MarkdownToHtmlResponse* response, std::function<void(::grpc::Status)>) = 0;
      virtual void ToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest* request, ::palm::morus::v1::MarkdownToHtmlResponse* response, ::grpc::ClientUnaryReactor* reactor) = 0;
    };
    typedef class async_interface experimental_async_interface;
    virtual class async_interface* async() { return nullptr; }
    class async_interface* experimental_async() { return async(); }
   private:
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::palm::morus::v1::MarkdownToHtmlResponse>* AsyncToHtmlRaw(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) = 0;
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::palm::morus::v1::MarkdownToHtmlResponse>* PrepareAsyncToHtmlRaw(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) = 0;
  };
  class Stub final : public StubInterface {
   public:
    Stub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options = ::grpc::StubOptions());
    ::grpc::Status ToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::palm::morus::v1::MarkdownToHtmlResponse* response) override;
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::morus::v1::MarkdownToHtmlResponse>> AsyncToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::morus::v1::MarkdownToHtmlResponse>>(AsyncToHtmlRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::morus::v1::MarkdownToHtmlResponse>> PrepareAsyncToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::morus::v1::MarkdownToHtmlResponse>>(PrepareAsyncToHtmlRaw(context, request, cq));
    }
    class async final :
      public StubInterface::async_interface {
     public:
      void ToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest* request, ::palm::morus::v1::MarkdownToHtmlResponse* response, std::function<void(::grpc::Status)>) override;
      void ToHtml(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest* request, ::palm::morus::v1::MarkdownToHtmlResponse* response, ::grpc::ClientUnaryReactor* reactor) override;
     private:
      friend class Stub;
      explicit async(Stub* stub): stub_(stub) { }
      Stub* stub() { return stub_; }
      Stub* stub_;
    };
    class async* async() override { return &async_stub_; }

   private:
    std::shared_ptr< ::grpc::ChannelInterface> channel_;
    class async async_stub_{this};
    ::grpc::ClientAsyncResponseReader< ::palm::morus::v1::MarkdownToHtmlResponse>* AsyncToHtmlRaw(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) override;
    ::grpc::ClientAsyncResponseReader< ::palm::morus::v1::MarkdownToHtmlResponse>* PrepareAsyncToHtmlRaw(::grpc::ClientContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest& request, ::grpc::CompletionQueue* cq) override;
    const ::grpc::internal::RpcMethod rpcmethod_ToHtml_;
  };
  static std::unique_ptr<Stub> NewStub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options = ::grpc::StubOptions());

  class Service : public ::grpc::Service {
   public:
    Service();
    virtual ~Service();
    virtual ::grpc::Status ToHtml(::grpc::ServerContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest* request, ::palm::morus::v1::MarkdownToHtmlResponse* response);
  };
  template <class BaseClass>
  class WithAsyncMethod_ToHtml : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithAsyncMethod_ToHtml() {
      ::grpc::Service::MarkMethodAsync(0);
    }
    ~WithAsyncMethod_ToHtml() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status ToHtml(::grpc::ServerContext* /*context*/, const ::palm::morus::v1::MarkdownToHtmlRequest* /*request*/, ::palm::morus::v1::MarkdownToHtmlResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestToHtml(::grpc::ServerContext* context, ::palm::morus::v1::MarkdownToHtmlRequest* request, ::grpc::ServerAsyncResponseWriter< ::palm::morus::v1::MarkdownToHtmlResponse>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(0, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  typedef WithAsyncMethod_ToHtml<Service > AsyncService;
  template <class BaseClass>
  class WithCallbackMethod_ToHtml : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithCallbackMethod_ToHtml() {
      ::grpc::Service::MarkMethodCallback(0,
          new ::grpc::internal::CallbackUnaryHandler< ::palm::morus::v1::MarkdownToHtmlRequest, ::palm::morus::v1::MarkdownToHtmlResponse>(
            [this](
                   ::grpc::CallbackServerContext* context, const ::palm::morus::v1::MarkdownToHtmlRequest* request, ::palm::morus::v1::MarkdownToHtmlResponse* response) { return this->ToHtml(context, request, response); }));}
    void SetMessageAllocatorFor_ToHtml(
        ::grpc::MessageAllocator< ::palm::morus::v1::MarkdownToHtmlRequest, ::palm::morus::v1::MarkdownToHtmlResponse>* allocator) {
      ::grpc::internal::MethodHandler* const handler = ::grpc::Service::GetHandler(0);
      static_cast<::grpc::internal::CallbackUnaryHandler< ::palm::morus::v1::MarkdownToHtmlRequest, ::palm::morus::v1::MarkdownToHtmlResponse>*>(handler)
              ->SetMessageAllocator(allocator);
    }
    ~WithCallbackMethod_ToHtml() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status ToHtml(::grpc::ServerContext* /*context*/, const ::palm::morus::v1::MarkdownToHtmlRequest* /*request*/, ::palm::morus::v1::MarkdownToHtmlResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    virtual ::grpc::ServerUnaryReactor* ToHtml(
      ::grpc::CallbackServerContext* /*context*/, const ::palm::morus::v1::MarkdownToHtmlRequest* /*request*/, ::palm::morus::v1::MarkdownToHtmlResponse* /*response*/)  { return nullptr; }
  };
  typedef WithCallbackMethod_ToHtml<Service > CallbackService;
  typedef CallbackService ExperimentalCallbackService;
  template <class BaseClass>
  class WithGenericMethod_ToHtml : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithGenericMethod_ToHtml() {
      ::grpc::Service::MarkMethodGeneric(0);
    }
    ~WithGenericMethod_ToHtml() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status ToHtml(::grpc::ServerContext* /*context*/, const ::palm::morus::v1::MarkdownToHtmlRequest* /*request*/, ::palm::morus::v1::MarkdownToHtmlResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
  };
  template <class BaseClass>
  class WithRawMethod_ToHtml : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawMethod_ToHtml() {
      ::grpc::Service::MarkMethodRaw(0);
    }
    ~WithRawMethod_ToHtml() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status ToHtml(::grpc::ServerContext* /*context*/, const ::palm::morus::v1::MarkdownToHtmlRequest* /*request*/, ::palm::morus::v1::MarkdownToHtmlResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestToHtml(::grpc::ServerContext* context, ::grpc::ByteBuffer* request, ::grpc::ServerAsyncResponseWriter< ::grpc::ByteBuffer>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(0, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  template <class BaseClass>
  class WithRawCallbackMethod_ToHtml : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawCallbackMethod_ToHtml() {
      ::grpc::Service::MarkMethodRawCallback(0,
          new ::grpc::internal::CallbackUnaryHandler< ::grpc::ByteBuffer, ::grpc::ByteBuffer>(
            [this](
                   ::grpc::CallbackServerContext* context, const ::grpc::ByteBuffer* request, ::grpc::ByteBuffer* response) { return this->ToHtml(context, request, response); }));
    }
    ~WithRawCallbackMethod_ToHtml() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status ToHtml(::grpc::ServerContext* /*context*/, const ::palm::morus::v1::MarkdownToHtmlRequest* /*request*/, ::palm::morus::v1::MarkdownToHtmlResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    virtual ::grpc::ServerUnaryReactor* ToHtml(
      ::grpc::CallbackServerContext* /*context*/, const ::grpc::ByteBuffer* /*request*/, ::grpc::ByteBuffer* /*response*/)  { return nullptr; }
  };
  template <class BaseClass>
  class WithStreamedUnaryMethod_ToHtml : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithStreamedUnaryMethod_ToHtml() {
      ::grpc::Service::MarkMethodStreamed(0,
        new ::grpc::internal::StreamedUnaryHandler<
          ::palm::morus::v1::MarkdownToHtmlRequest, ::palm::morus::v1::MarkdownToHtmlResponse>(
            [this](::grpc::ServerContext* context,
                   ::grpc::ServerUnaryStreamer<
                     ::palm::morus::v1::MarkdownToHtmlRequest, ::palm::morus::v1::MarkdownToHtmlResponse>* streamer) {
                       return this->StreamedToHtml(context,
                         streamer);
                  }));
    }
    ~WithStreamedUnaryMethod_ToHtml() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable regular version of this method
    ::grpc::Status ToHtml(::grpc::ServerContext* /*context*/, const ::palm::morus::v1::MarkdownToHtmlRequest* /*request*/, ::palm::morus::v1::MarkdownToHtmlResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    // replace default version of method with streamed unary
    virtual ::grpc::Status StreamedToHtml(::grpc::ServerContext* context, ::grpc::ServerUnaryStreamer< ::palm::morus::v1::MarkdownToHtmlRequest,::palm::morus::v1::MarkdownToHtmlResponse>* server_unary_streamer) = 0;
  };
  typedef WithStreamedUnaryMethod_ToHtml<Service > StreamedUnaryService;
  typedef Service SplitStreamedService;
  typedef WithStreamedUnaryMethod_ToHtml<Service > StreamedService;
};

}  // namespace v1
}  // namespace morus
}  // namespace palm


#endif  // GRPC_morus_2eproto__INCLUDED
