defmodule Jasmine.Utils.HMac do
  require Logger

  def sign(p) do
    {:ok, key} = Application.get_env(:jasmine, HMac)[:key] |> Base.decode64()
    :crypto.mac(:hmac, :sha512, key, p)
  end

  def verify?(c, p) do
    c == sign(p)
  end
end
