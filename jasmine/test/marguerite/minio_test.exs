defmodule Marguerite.MinioTest do
  use ExUnit.Case

  require Logger

  test "minio" do
    bucket = "marguerite.testing"
    assert Marguerite.NIF.s3_create_bucket(bucket, false, 30)

    file = "mix.lock"
    {object, size} = Marguerite.NIF.s3_put_object(bucket, file)
    Logger.info("upload #{file}(#{size} bytes) to (#{bucket}, #{object})")

    url = Marguerite.NIF.s3_get_presigned_object_url(bucket, object)
    Logger.info("show object: #{url}")
  end
end
