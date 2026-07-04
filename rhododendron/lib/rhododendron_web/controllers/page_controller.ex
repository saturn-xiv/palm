defmodule RhododendronWeb.PageController do
  use RhododendronWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
