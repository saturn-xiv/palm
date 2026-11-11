defmodule Bougainvillea.Portal.Controllers do
  def header(conn, key) do
    key = key |> String.downcase()

    case Plug.Conn.get_req_header(conn, key) do
      [v | _] ->
        {:ok, v}

      _ ->
        {:error, "Couldn't found http header #{key}"}
    end
  end
end
