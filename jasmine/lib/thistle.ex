defmodule Thistle.NIF do
  @on_load :init
  def init do
    path = :filename.join([:code.priv_dir(:jasmine), "libthistle"])
    :erlang.load_nif(path, 0)
  end

  def add_roles_for_user(_x, _y), do: :erlang.nif_error(:nif_library_not_loaded)

  def version(), do: :erlang.nif_error(:nif_library_not_loaded)
end
