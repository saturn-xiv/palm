defmodule Marigold do
  def open() do
    GRPC.Stub.connect(
      Application.fetch_env!(:bougainvillea, :marigold)
      |> Keyword.fetch!(:endpoint)
    )
  end
end
