defmodule Loquat.Jwt do
  # https://en.wikipedia.org/wiki/ISO_8601#Durations
  def sign(issuer, subject, audiences, payload, ttl) do
    {:ok, ttl} = Duration.from_iso8601(ttl)
    {:ok, ch} = Loquat.open()
    now = DateTime.utc_now(:microsecond)

    {:ok, res} =
      Palm.Loquat.V1.JwtService.Stub.compute(ch, %Palm.Loquat.V1.JwtComputeRequest{
        subject: subject,
        audiences: audiences,
        payload: payload |> Protobuf.encode(),
        not_before: now |> DateTime.to_unix(),
        issued_at: now |> DateTime.to_unix(),
        expired_at: DateTime.shift(now, ttl) |> DateTime.to_unix(),
        issuer: issuer
      })

    {:ok, res.token}
  end

  def verify(token, issuer, audience) do
    {:ok, ch} = Loquat.open()

    {:ok, res} =
      Palm.Loquat.V1.JwtService.Stub.verify(ch, %Palm.Loquat.V1.JwtVerifyRequest{
        token: token,
        audience: audience,
        issuer: issuer
      })

    {:ok, res.subject, res.payload}
  end
end
