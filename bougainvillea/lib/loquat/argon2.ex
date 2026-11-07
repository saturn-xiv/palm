defmodule Loquat.Argon2 do
  def sign(password) do
    {:ok, ch} = Loquat.open()

    {:ok, res} =
      Palm.Loquat.V1.Argon2Service.Stub.compute(ch, %Palm.Loquat.V1.Argon2ComputeRequest{password: password})

    {:ok, res.hashed}
  end

  def verify?(hashed, password) do
    {:ok, ch} = Loquat.open()

    {:ok, _} =
      Palm.Loquat.V1.Argon2Service.Stub.verify(ch, %Palm.Loquat.V1.Argon2VerifyRequest{
        hashed: hashed,
        password: password
      })

    true
  end
end
