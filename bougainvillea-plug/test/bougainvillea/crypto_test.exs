defmodule BougainvilleaTest.Crypto do
  use ExUnit.Case
  @hi "Hello, Bougainvillea!"

  test "uuid v4" do
    Enum.each(1..5, fn i ->
      it = Hibiscus.uuid()
      IO.puts("uuid v4(#{i}): #{it}")
    end)
  end

  test "random alphanumeric" do
    Enum.each(1..5, fn i ->
      it = Hibiscus.Random.alphanumeric(16)
      IO.puts("random alphanumeric(#{i}): #{it}")
    end)
  end

  test "random bytes" do
    Enum.each(1..5, fn i ->
      it = Hibiscus.Random.bytes(16) |> Base.url_encode64(padding: false)
      IO.puts("random bytes(#{i}): #{it}")
    end)
  end

  test "sha256" do
    Enum.each(1..3, fn i ->
      it = Hibiscus.Sha256.compute(@hi)
      IO.puts("sha256(#{@hi}, #{i}): #{it}")
    end)
  end

  test "sha512" do
    Enum.each(1..3, fn i ->
      it = Hibiscus.Sha512.compute(@hi)
      IO.puts("sha512(#{@hi}, #{i}): #{it}")
    end)
  end

  test "gravatar" do
    Enum.each(
      [
        "myemailaddress@example.com",
        "MyEmailAddress@example.com",
        " MyEmailAddress@example.com",
        "MyEmailAddress@example.com ",
        " MyEmailAddress@example.com "
      ],
      fn i ->
        assert "84059b07d4be67b806386c0aad8070a23f18836bbaae342275dc0a83414c32ee" ==
                 Hibiscus.Gravatar.hash(i)
      end
    )
  end

  test "ssha512" do
    Enum.each(1..5, fn i ->
      hash = Hibiscus.Ssha512.sign(@hi, 8)
      IO.puts("SSHA512(#{i}): doveadm pw -t '#{hash}' -p '#{@hi}'")
      assert Hibiscus.Ssha512.verify(hash, @hi)
    end)

    assert !Hibiscus.Ssha512.verify("Hello, World!", @hi)
  end
end
