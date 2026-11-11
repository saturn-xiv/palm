defmodule Dahlia do
  def open() do
    GRPC.Stub.connect(
      Application.fetch_env!(:bougainvillea, :dahlia)
      |> Keyword.fetch!(:endpoint)
    )
  end
end
