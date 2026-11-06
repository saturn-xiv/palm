defmodule Bougainvillea.Portal do
  def default_lang() do
    "en-US"
  end

  def default_timezone() do
    "UTC"
  end

  def token_issuer() do
    Application.fetch_env!(:bougainvillea, :token)
    |> Keyword.fetch!(:issuer)
  end
end
