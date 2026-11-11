defmodule Mix.Tasks.Bougainvillea.SetPasswordForEmailUser do
  use Mix.Task

  @shortdoc "Set password for email user(-e/--email, -p/--password)"

  @impl true
  def run(args) do
    Mix.Task.run("app.start")

    {parsed_options, _remaining_args, invalid_options} =
      OptionParser.parse(args,
        strict: [email: :string, password: :string],
        aliases: [e: :email, p: :password]
      )

    if invalid_options != [] do
      Mix.shell().error("Invalid options provided: #{inspect(invalid_options)}")
    end

    email = Keyword.get(parsed_options, :email)
    password = Keyword.get(parsed_options, :password)
    Bougainvillea.Tasks.User.set_password_for_email_user(email, password)
  end
end
