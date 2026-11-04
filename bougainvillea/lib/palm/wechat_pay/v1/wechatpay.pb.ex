defmodule Palm.WechatPay.V1.BillPullRequest do
  @moduledoc false

  use Protobuf,
    full_name: "palm.wechat_pay.v1.BillPullRequest",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.WechatPay.V1.BillPullResponse do
  @moduledoc false

  use Protobuf,
    full_name: "palm.wechat_pay.v1.BillPullResponse",
    protoc_gen_elixir_version: "0.17.1",
    syntax: :proto3
end

defmodule Palm.WechatPay.V1.WeChatPay.Service do
  @moduledoc false

  use GRPC.Service, name: "palm.wechat_pay.v1.WeChatPay", protoc_gen_elixir_version: "0.17.1"

  rpc(:BillPull, Palm.WechatPay.V1.BillPullRequest, Palm.WechatPay.V1.BillPullResponse)
end

defmodule Palm.WechatPay.V1.WeChatPay.Stub do
  @moduledoc false

  use GRPC.Stub, service: Palm.WechatPay.V1.WeChatPay.Service
end
