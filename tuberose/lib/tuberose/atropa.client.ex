defmodule Tuberose.Atropa.Client do
  def connect() do
    GRPC.Stub.connect(Application.get_env(:tuberose, Tuberose.Atropa)[:host])
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

  def hmac_sign(password, salt_len) do
    salt = :crypto.strong_rand_bytes(salt_len)
    {:ok, channel} = connect()
    request = %Palm.Atropa.V1.HMacSignRequest{plain: password <> salt}
    {:ok, reply} = channel |> Palm.Atropa.V1.HMac.Stub.sign(request)
    {reply.code, salt}
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
end
