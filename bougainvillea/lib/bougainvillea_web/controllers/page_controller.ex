defmodule BougainvilleaWeb.PageController do
  use BougainvilleaWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
