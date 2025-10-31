defmodule Mix.Tasks.Bamboo.SetAdministrator do
  use Mix.Task

  require Logger
  require OptionParser
  require Validate
  import Ecto.Changeset

  @shortdoc "Setup an administrator account"
  def run(args) do
    Mix.Task.run "app.start"

    options = OptionParser.parse(args, strict: [name: :string, password: :string], aliases: [n: :name, p: :password])
    Logger.debug "receive arguments: #{inspect(options)}"

    case options do
      { [{:name, name}, {:password, password}], _, _ } ->
        user = %Bamboo.User{name: name, password: password}
        {:ok, _} = Validate.validate(user, Bamboo.User.rules)
        user = %{user | password: Bamboo.User.generate_password("mysecretkey", user.password)}

        case Bamboo.Repo.get_by(Bamboo.User, name: user.name) do
          nil ->
            Logger.info "create administrator #{user.name}"
            Bamboo.Repo.insert(user)
            it = Bamboo.Repo.get_by(Bamboo.User, name: user.name)
            Bamboo.Repo.insert(%Bamboo.Log{user_id: it.id, level: to_string(:info), message: "created."})
          it ->
            Bamboo.Repo.transaction(fn ->
              Logger.info "update administrator #{user.name}'s password"
              Bamboo.Repo.update(change(it, %{password: user.password, version: it.version+1}))
              Bamboo.Repo.insert(%Bamboo.Log{user_id: it.id, level: to_string(:info), message: "reset password."})
            end)
        end

        # Logger.debug "found user #{inspect(it)}"

      _ ->
        Logger.error "Usage: --name/-n NAME --password/-p PASSWORD"
    end

  end

end
