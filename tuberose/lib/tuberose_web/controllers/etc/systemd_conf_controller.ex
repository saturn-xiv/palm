defmodule TuberoseWeb.Etc.SystemdConfController do
  use TuberoseWeb, :controller

  def get(conn, %{"domain" => domain, "port" => port}) do
    text(conn, """
    # /usr/lib/systemd/system/#{domain}.conf
    [Unit]
    Description=A total free education & translation solution(#{domain}).
    Wants=network-online.target
    After=network-online.target

    [Service]
    Type=notify
    User=www-data
    Group=www-data
    ExecStart=bin/server
    WorkingDirectory=/var/www/#{domain}
    Restart=always
    RestartSec=10s
    Environment="SECRET_KEY_BASE='$(mix phx.gen.secret)'"
    Environment="DATABASE_URL='postgres://www:$(pwgen 32 1)@127.0.0.1/palm?sslmode=disable'"
    Environment="ATROPA_HOST=127.0.0.1:9999"
    Environment="PORT=#{port}"

    [Install]
    WantedBy=multi-user.target
    """)
  end
end
