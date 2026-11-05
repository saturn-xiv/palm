defmodule Bougainvillea do
  def token_issuer() do
    Application.fetch_env!(:bougainvillea, :token)
    |> Keyword.fetch!(:issuer)
  end
end
