defmodule BambooWeb.PageController do
  use BambooWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
