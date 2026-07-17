defmodule RhododendronWeb.Resolvers.Portal.User do
  require Logger

  def sign_out(
        _parent,
        _args,
        %{
          context: %RhododendronWeb.Session{token: token, client_ip: client_ip}
        }
      ) do
    # TODO
    Logger.info("User sign out")
    {:ok, RhododendronWeb.Portal.ok()}
  end


end
