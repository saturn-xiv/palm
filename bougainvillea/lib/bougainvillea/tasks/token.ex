defmodule Bougainvillea.Tasks.Token do
  require Logger

  def generate_for_email_user(email, audiences, ttl) do
    {:ok, email} = Hibiscus.Validator.email(email)
    it = Bougainvillea.Repo.get_by(Bougainvillea.Portal.Models.EmailUser, email: email)

    Logger.warning(
      "Generate a #{ttl} days token with audiences(#{Enum.join(audiences, ",")}) for #{it.name}<#{it.email}>"
    )

    {:ok, token} =
      Loquat.Jwt.sign(
        Bougainvillea.token_issuer(),
        it.sn,
        audiences,
        %Palm.Portal.V1.UserToken.Payload{
          type: :EMAIL
        },
        ttl
      )

    IO.puts("TOKEN: #{token}")
    Logger.info("done.")
  end
end
