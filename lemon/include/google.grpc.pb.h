// Generated by the gRPC C++ plugin.
// If you make any local change, they will be lost.
// source: google.proto
#ifndef GRPC_google_2eproto__INCLUDED
#define GRPC_google_2eproto__INCLUDED

#include "google.pb.h"

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
namespace google {
namespace v1 {

class Oauth2 final {
 public:
  static constexpr char const* service_full_name() {
    return "palm.google.v1.Oauth2";
  }
  class StubInterface {
   public:
    virtual ~StubInterface() {}
    virtual ::grpc::Status AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response) = 0;
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2AuthCodeURLResponse>> AsyncAuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2AuthCodeURLResponse>>(AsyncAuthCodeURLRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2AuthCodeURLResponse>> PrepareAsyncAuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2AuthCodeURLResponse>>(PrepareAsyncAuthCodeURLRaw(context, request, cq));
    }
    virtual ::grpc::Status SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::palm::google::v1::Oauth2SignInResponse* response) = 0;
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2SignInResponse>> AsyncSignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2SignInResponse>>(AsyncSignInRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2SignInResponse>> PrepareAsyncSignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2SignInResponse>>(PrepareAsyncSignInRaw(context, request, cq));
    }
    class async_interface {
     public:
      virtual ~async_interface() {}
      virtual void AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response, std::function<void(::grpc::Status)>) = 0;
      virtual void AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response, ::grpc::ClientUnaryReactor* reactor) = 0;
      virtual void SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response, std::function<void(::grpc::Status)>) = 0;
      virtual void SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response, ::grpc::ClientUnaryReactor* reactor) = 0;
    };
    typedef class async_interface experimental_async_interface;
    virtual class async_interface* async() { return nullptr; }
    class async_interface* experimental_async() { return async(); }
   private:
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2AuthCodeURLResponse>* AsyncAuthCodeURLRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) = 0;
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2AuthCodeURLResponse>* PrepareAsyncAuthCodeURLRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) = 0;
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2SignInResponse>* AsyncSignInRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) = 0;
    virtual ::grpc::ClientAsyncResponseReaderInterface< ::palm::google::v1::Oauth2SignInResponse>* PrepareAsyncSignInRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) = 0;
  };
  class Stub final : public StubInterface {
   public:
    Stub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options = ::grpc::StubOptions());
    ::grpc::Status AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response) override;
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>> AsyncAuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>>(AsyncAuthCodeURLRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>> PrepareAsyncAuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>>(PrepareAsyncAuthCodeURLRaw(context, request, cq));
    }
    ::grpc::Status SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::palm::google::v1::Oauth2SignInResponse* response) override;
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>> AsyncSignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>>(AsyncSignInRaw(context, request, cq));
    }
    std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>> PrepareAsyncSignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) {
      return std::unique_ptr< ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>>(PrepareAsyncSignInRaw(context, request, cq));
    }
    class async final :
      public StubInterface::async_interface {
     public:
      void AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response, std::function<void(::grpc::Status)>) override;
      void AuthCodeURL(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response, ::grpc::ClientUnaryReactor* reactor) override;
      void SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response, std::function<void(::grpc::Status)>) override;
      void SignIn(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response, ::grpc::ClientUnaryReactor* reactor) override;
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
    ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>* AsyncAuthCodeURLRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) override;
    ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2AuthCodeURLResponse>* PrepareAsyncAuthCodeURLRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest& request, ::grpc::CompletionQueue* cq) override;
    ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>* AsyncSignInRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) override;
    ::grpc::ClientAsyncResponseReader< ::palm::google::v1::Oauth2SignInResponse>* PrepareAsyncSignInRaw(::grpc::ClientContext* context, const ::palm::google::v1::Oauth2SignInRequest& request, ::grpc::CompletionQueue* cq) override;
    const ::grpc::internal::RpcMethod rpcmethod_AuthCodeURL_;
    const ::grpc::internal::RpcMethod rpcmethod_SignIn_;
  };
  static std::unique_ptr<Stub> NewStub(const std::shared_ptr< ::grpc::ChannelInterface>& channel, const ::grpc::StubOptions& options = ::grpc::StubOptions());

  class Service : public ::grpc::Service {
   public:
    Service();
    virtual ~Service();
    virtual ::grpc::Status AuthCodeURL(::grpc::ServerContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response);
    virtual ::grpc::Status SignIn(::grpc::ServerContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response);
  };
  template <class BaseClass>
  class WithAsyncMethod_AuthCodeURL : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithAsyncMethod_AuthCodeURL() {
      ::grpc::Service::MarkMethodAsync(0);
    }
    ~WithAsyncMethod_AuthCodeURL() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status AuthCodeURL(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2AuthCodeURLRequest* /*request*/, ::palm::google::v1::Oauth2AuthCodeURLResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestAuthCodeURL(::grpc::ServerContext* context, ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::grpc::ServerAsyncResponseWriter< ::palm::google::v1::Oauth2AuthCodeURLResponse>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(0, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  template <class BaseClass>
  class WithAsyncMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithAsyncMethod_SignIn() {
      ::grpc::Service::MarkMethodAsync(1);
    }
    ~WithAsyncMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2SignInRequest* /*request*/, ::palm::google::v1::Oauth2SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestSignIn(::grpc::ServerContext* context, ::palm::google::v1::Oauth2SignInRequest* request, ::grpc::ServerAsyncResponseWriter< ::palm::google::v1::Oauth2SignInResponse>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(1, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  typedef WithAsyncMethod_AuthCodeURL<WithAsyncMethod_SignIn<Service > > AsyncService;
  template <class BaseClass>
  class WithCallbackMethod_AuthCodeURL : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithCallbackMethod_AuthCodeURL() {
      ::grpc::Service::MarkMethodCallback(0,
          new ::grpc::internal::CallbackUnaryHandler< ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse>(
            [this](
                   ::grpc::CallbackServerContext* context, const ::palm::google::v1::Oauth2AuthCodeURLRequest* request, ::palm::google::v1::Oauth2AuthCodeURLResponse* response) { return this->AuthCodeURL(context, request, response); }));}
    void SetMessageAllocatorFor_AuthCodeURL(
        ::grpc::MessageAllocator< ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse>* allocator) {
      ::grpc::internal::MethodHandler* const handler = ::grpc::Service::GetHandler(0);
      static_cast<::grpc::internal::CallbackUnaryHandler< ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse>*>(handler)
              ->SetMessageAllocator(allocator);
    }
    ~WithCallbackMethod_AuthCodeURL() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status AuthCodeURL(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2AuthCodeURLRequest* /*request*/, ::palm::google::v1::Oauth2AuthCodeURLResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    virtual ::grpc::ServerUnaryReactor* AuthCodeURL(
      ::grpc::CallbackServerContext* /*context*/, const ::palm::google::v1::Oauth2AuthCodeURLRequest* /*request*/, ::palm::google::v1::Oauth2AuthCodeURLResponse* /*response*/)  { return nullptr; }
  };
  template <class BaseClass>
  class WithCallbackMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithCallbackMethod_SignIn() {
      ::grpc::Service::MarkMethodCallback(1,
          new ::grpc::internal::CallbackUnaryHandler< ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse>(
            [this](
                   ::grpc::CallbackServerContext* context, const ::palm::google::v1::Oauth2SignInRequest* request, ::palm::google::v1::Oauth2SignInResponse* response) { return this->SignIn(context, request, response); }));}
    void SetMessageAllocatorFor_SignIn(
        ::grpc::MessageAllocator< ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse>* allocator) {
      ::grpc::internal::MethodHandler* const handler = ::grpc::Service::GetHandler(1);
      static_cast<::grpc::internal::CallbackUnaryHandler< ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse>*>(handler)
              ->SetMessageAllocator(allocator);
    }
    ~WithCallbackMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2SignInRequest* /*request*/, ::palm::google::v1::Oauth2SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    virtual ::grpc::ServerUnaryReactor* SignIn(
      ::grpc::CallbackServerContext* /*context*/, const ::palm::google::v1::Oauth2SignInRequest* /*request*/, ::palm::google::v1::Oauth2SignInResponse* /*response*/)  { return nullptr; }
  };
  typedef WithCallbackMethod_AuthCodeURL<WithCallbackMethod_SignIn<Service > > CallbackService;
  typedef CallbackService ExperimentalCallbackService;
  template <class BaseClass>
  class WithGenericMethod_AuthCodeURL : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithGenericMethod_AuthCodeURL() {
      ::grpc::Service::MarkMethodGeneric(0);
    }
    ~WithGenericMethod_AuthCodeURL() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status AuthCodeURL(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2AuthCodeURLRequest* /*request*/, ::palm::google::v1::Oauth2AuthCodeURLResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
  };
  template <class BaseClass>
  class WithGenericMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithGenericMethod_SignIn() {
      ::grpc::Service::MarkMethodGeneric(1);
    }
    ~WithGenericMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2SignInRequest* /*request*/, ::palm::google::v1::Oauth2SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
  };
  template <class BaseClass>
  class WithRawMethod_AuthCodeURL : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawMethod_AuthCodeURL() {
      ::grpc::Service::MarkMethodRaw(0);
    }
    ~WithRawMethod_AuthCodeURL() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status AuthCodeURL(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2AuthCodeURLRequest* /*request*/, ::palm::google::v1::Oauth2AuthCodeURLResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestAuthCodeURL(::grpc::ServerContext* context, ::grpc::ByteBuffer* request, ::grpc::ServerAsyncResponseWriter< ::grpc::ByteBuffer>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(0, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  template <class BaseClass>
  class WithRawMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawMethod_SignIn() {
      ::grpc::Service::MarkMethodRaw(1);
    }
    ~WithRawMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2SignInRequest* /*request*/, ::palm::google::v1::Oauth2SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    void RequestSignIn(::grpc::ServerContext* context, ::grpc::ByteBuffer* request, ::grpc::ServerAsyncResponseWriter< ::grpc::ByteBuffer>* response, ::grpc::CompletionQueue* new_call_cq, ::grpc::ServerCompletionQueue* notification_cq, void *tag) {
      ::grpc::Service::RequestAsyncUnary(1, context, request, response, new_call_cq, notification_cq, tag);
    }
  };
  template <class BaseClass>
  class WithRawCallbackMethod_AuthCodeURL : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawCallbackMethod_AuthCodeURL() {
      ::grpc::Service::MarkMethodRawCallback(0,
          new ::grpc::internal::CallbackUnaryHandler< ::grpc::ByteBuffer, ::grpc::ByteBuffer>(
            [this](
                   ::grpc::CallbackServerContext* context, const ::grpc::ByteBuffer* request, ::grpc::ByteBuffer* response) { return this->AuthCodeURL(context, request, response); }));
    }
    ~WithRawCallbackMethod_AuthCodeURL() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status AuthCodeURL(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2AuthCodeURLRequest* /*request*/, ::palm::google::v1::Oauth2AuthCodeURLResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    virtual ::grpc::ServerUnaryReactor* AuthCodeURL(
      ::grpc::CallbackServerContext* /*context*/, const ::grpc::ByteBuffer* /*request*/, ::grpc::ByteBuffer* /*response*/)  { return nullptr; }
  };
  template <class BaseClass>
  class WithRawCallbackMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithRawCallbackMethod_SignIn() {
      ::grpc::Service::MarkMethodRawCallback(1,
          new ::grpc::internal::CallbackUnaryHandler< ::grpc::ByteBuffer, ::grpc::ByteBuffer>(
            [this](
                   ::grpc::CallbackServerContext* context, const ::grpc::ByteBuffer* request, ::grpc::ByteBuffer* response) { return this->SignIn(context, request, response); }));
    }
    ~WithRawCallbackMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable synchronous version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2SignInRequest* /*request*/, ::palm::google::v1::Oauth2SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    virtual ::grpc::ServerUnaryReactor* SignIn(
      ::grpc::CallbackServerContext* /*context*/, const ::grpc::ByteBuffer* /*request*/, ::grpc::ByteBuffer* /*response*/)  { return nullptr; }
  };
  template <class BaseClass>
  class WithStreamedUnaryMethod_AuthCodeURL : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithStreamedUnaryMethod_AuthCodeURL() {
      ::grpc::Service::MarkMethodStreamed(0,
        new ::grpc::internal::StreamedUnaryHandler<
          ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse>(
            [this](::grpc::ServerContext* context,
                   ::grpc::ServerUnaryStreamer<
                     ::palm::google::v1::Oauth2AuthCodeURLRequest, ::palm::google::v1::Oauth2AuthCodeURLResponse>* streamer) {
                       return this->StreamedAuthCodeURL(context,
                         streamer);
                  }));
    }
    ~WithStreamedUnaryMethod_AuthCodeURL() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable regular version of this method
    ::grpc::Status AuthCodeURL(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2AuthCodeURLRequest* /*request*/, ::palm::google::v1::Oauth2AuthCodeURLResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    // replace default version of method with streamed unary
    virtual ::grpc::Status StreamedAuthCodeURL(::grpc::ServerContext* context, ::grpc::ServerUnaryStreamer< ::palm::google::v1::Oauth2AuthCodeURLRequest,::palm::google::v1::Oauth2AuthCodeURLResponse>* server_unary_streamer) = 0;
  };
  template <class BaseClass>
  class WithStreamedUnaryMethod_SignIn : public BaseClass {
   private:
    void BaseClassMustBeDerivedFromService(const Service* /*service*/) {}
   public:
    WithStreamedUnaryMethod_SignIn() {
      ::grpc::Service::MarkMethodStreamed(1,
        new ::grpc::internal::StreamedUnaryHandler<
          ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse>(
            [this](::grpc::ServerContext* context,
                   ::grpc::ServerUnaryStreamer<
                     ::palm::google::v1::Oauth2SignInRequest, ::palm::google::v1::Oauth2SignInResponse>* streamer) {
                       return this->StreamedSignIn(context,
                         streamer);
                  }));
    }
    ~WithStreamedUnaryMethod_SignIn() override {
      BaseClassMustBeDerivedFromService(this);
    }
    // disable regular version of this method
    ::grpc::Status SignIn(::grpc::ServerContext* /*context*/, const ::palm::google::v1::Oauth2SignInRequest* /*request*/, ::palm::google::v1::Oauth2SignInResponse* /*response*/) override {
      abort();
      return ::grpc::Status(::grpc::StatusCode::UNIMPLEMENTED, "");
    }
    // replace default version of method with streamed unary
    virtual ::grpc::Status StreamedSignIn(::grpc::ServerContext* context, ::grpc::ServerUnaryStreamer< ::palm::google::v1::Oauth2SignInRequest,::palm::google::v1::Oauth2SignInResponse>* server_unary_streamer) = 0;
  };
  typedef WithStreamedUnaryMethod_AuthCodeURL<WithStreamedUnaryMethod_SignIn<Service > > StreamedUnaryService;
  typedef Service SplitStreamedService;
  typedef WithStreamedUnaryMethod_AuthCodeURL<WithStreamedUnaryMethod_SignIn<Service > > StreamedService;
};

}  // namespace v1
}  // namespace google
}  // namespace palm


#endif  // GRPC_google_2eproto__INCLUDED
