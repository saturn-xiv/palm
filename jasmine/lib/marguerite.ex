defmodule Marguerite.NIF do
  @on_load :init
  def init do
    path = :filename.join([:code.priv_dir(:jasmine), "libmarguerite"])
    :erlang.load_nif(path, 0)
  end

  def jwt_sign(_v, _w, _x, _y, _z), do: :erlang.nif_error(:nif_library_not_loaded)
  def jwt_sign(_u, _v, _w, _x, _y, _z), do: :erlang.nif_error(:nif_library_not_loaded)
  def jwt_verify(_x, _y, _z), do: :erlang.nif_error(:nif_library_not_loaded)

  def hmac_sign(_x), do: :erlang.nif_error(:nif_library_not_loaded)
  def hmac_verify(_x, _y), do: :erlang.nif_error(:nif_library_not_loaded)

  def aes_encrypt(_x), do: :erlang.nif_error(:nif_library_not_loaded)
  def aes_decrypt(_x), do: :erlang.nif_error(:nif_library_not_loaded)

  def s3_create_bucket(_x), do: :erlang.nif_error(:nif_library_not_loaded)
  def s3_create_bucket(_x, _y), do: :erlang.nif_error(:nif_library_not_loaded)
  def s3_create_bucket(_x, _y, _z), do: :erlang.nif_error(:nif_library_not_loaded)
  def s3_put_object(_x, _y), do: :erlang.nif_error(:nif_library_not_loaded)
  def s3_get_presigned_object_url(_x, _y), do: :erlang.nif_error(:nif_library_not_loaded)
  def s3_get_presigned_object_url(_x, _y, _z), do: :erlang.nif_error(:nif_library_not_loaded)
  def s3_get_permanent_object_url(_x, _y), do: :erlang.nif_error(:nif_library_not_loaded)

  def version(), do: :erlang.nif_error(:nif_library_not_loaded)
end
