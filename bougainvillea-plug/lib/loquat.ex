defmodule Loquat do
  def open() do
    GRPC.Stub.connect(
      Application.fetch_env!(:bougainvillea, :loquat)
      |> Keyword.fetch!(:endpoint)
    )
  end
end
