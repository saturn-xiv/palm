defmodule Tuberose.Atropa.Client do
  def administrator do
    %Palm.Atropa.V1.PolicyRolesResponse.Item{
      by: {:root, %Palm.Atropa.V1.PolicyRolesResponse.Item.Root{}}
    }
  end

  def root do
    %Palm.Atropa.V1.PolicyRolesResponse.Item{
      by: {:administrator, %Palm.Atropa.V1.PolicyRolesResponse.Item.Administrator{}}
    }
  end

  def role(code) do
    %Palm.Atropa.V1.PolicyRolesResponse.Item{
      by: {:code, code}
    }
  end

  def administrator?(user) do
    has?(user, administrator())
  end

  def root?(user) do
    has?(user, root())
  end

  def has?(user, role) do
    {:ok, channel} = connect()

    request = %Palm.Atropa.V1.PolicyHasRequest{
      user: to_user_subject(user),
      role: role
    }

    case channel |> Palm.Atropa.V1.Policy.Stub.has(request) do
      {:ok, _} -> true
      _ -> false
    end
  end

  def roles(user) do
    {:ok, channel} = connect()
    request = to_user_subject(user)
    {:ok, reply} = channel |> Palm.Atropa.V1.Policy.Stub.get_roles_for_user(request)
    to_original_roles(reply.items)
  end

  def implicit_roles(user) do
    {:ok, channel} = connect()
    request = to_user_subject(user)
    {:ok, reply} = channel |> Palm.Atropa.V1.Policy.Stub.get_implicit_roles_for_user(request)
    to_original_roles(reply.items)
  end

  def permissions(user) do
    {:ok, channel} = connect()
    request = to_user_subject(user)
    {:ok, reply} = channel |> Palm.Atropa.V1.Policy.Stub.get_permissions_for_user(request)
    to_original_permissions(reply.items)
  end

  def implicit_permissions(user) do
    {:ok, channel} = connect()
    request = to_user_subject(user)

    {:ok, reply} =
      channel |> Palm.Atropa.V1.Policy.Stub.get_implicit_permissions_for_user(request)

    to_original_permissions(reply.items)
  end

  def jwt_sign(issuer, subject, audiences, not_before, expires_at) do
    {:ok, channel} = connect()

    request = %Palm.Atropa.V1.JwtSignRequest{
      issuer: issuer,
      subject: subject,
      audiences: audiences,
      not_before: not_before,
      expires_at: expires_at
    }

    {:ok, reply} = channel |> Palm.Atropa.V1.Jwt.Stub.sign(request)
    reply.token
  end

  def jwt_verify(token, issuer, audience) do
    {:ok, channel} = connect()

    request = %Palm.Atropa.V1.JwtVerifyRequest{
      token: token,
      issuer: issuer,
      audience: audience
    }

    case channel |> Palm.Atropa.V1.Jwt.Stub.verify(request) do
      {:ok, reply} -> {:ok, reply.subject, reply.extra}
      error -> error
    end
  end

  def hmac_sign(password, salt_len) do
    salt = :crypto.strong_rand_bytes(salt_len)
    {:ok, channel} = connect()
    request = %Palm.Atropa.V1.HMacSignRequest{plain: password <> salt}
    {:ok, reply} = channel |> Palm.Atropa.V1.HMac.Stub.sign(request)
    {reply.code, salt}
  end

  def hmac_verify(code, password, salt) do
    {:ok, channel} = connect()
    request = %Palm.Atropa.V1.HMacVerifyRequest{code: code, plain: password <> salt}
    {:ok, _} = channel |> Palm.Atropa.V1.HMac.Stub.verify(request)
  end

  def aes_encrypt(plain) do
    {:ok, channel} = connect()
    request = %Palm.Atropa.V1.AesPlainMessage{payload: plain}
    {:ok, reply} = channel |> Palm.Atropa.V1.Aes.Stub.encrypt(request)
    {reply.payload, reply.salt}
  end

  def aes_decrypt(code, salt) do
    {:ok, channel} = connect()
    request = %Palm.Atropa.V1.AesCodeMessage{payload: code, salt: salt}
    {:ok, reply} = channel |> Palm.Atropa.V1.Aes.Stub.decrypt(request)
    reply.payload
  end

  def add_roles_for_user(user, roles) do
    {:ok, channel} = connect()

    request = %Palm.Atropa.V1.PolicyRolesForUserRequest{user: user, roles: roles}

    {:ok, _} = channel |> Palm.Atropa.V1.Policy.Stub.add_roles_for_user(request)
  end

  def delete_roles_for_user(user, roles) do
    {:ok, channel} = connect()

    request = %Palm.Atropa.V1.PolicyRolesForUserRequest{user: user, roles: roles}

    {:ok, _} = channel |> Palm.Atropa.V1.Policy.Stub.delete_roles_for_user(request)
  end

  defp connect() do
    GRPC.Stub.connect(Application.get_env(:tuberose, Tuberose.AtropaClient)[:host])
  end

  defp to_user_subject(user) do
    %Palm.Atropa.V1.PolicyUsersResponse.Item{
      id: {:i, user}
    }
  end

  defp to_original_roles(items) do
    items
    |> Enum.map(fn x ->
      case x do
        %Palm.Atropa.V1.PolicyRolesResponse.Item{
          by: {:code, code}
        } ->
          code

        _ ->
          nil
      end
    end)
    |> Enum.filter(fn x -> x end)
  end

  defp to_original_permissions(items) do
    items
    |> Enum.map(fn x ->
      case x.resource.id do
        %Palm.Atropa.V1.PolicyPermissionsResponse.Item.Resource.Id{
          by: {:i, id}
        } ->
          %{operation: x.operation, resource: %{type: x.resource.type, id: id}}

        nil ->
          %{operation: x.operation, resource: %{type: x.resource.type, id: nil}}

        _ ->
          nil
      end
    end)
    |> Enum.filter(fn x -> x end)
  end
end
