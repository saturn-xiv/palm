defmodule BambooWeb.EtcController do
  use BambooWeb, :controller

  # https://www.linode.com/docs/guides/use-nginx-reverse-proxy/
  def nginx_conf(conn, _params) do
    # /etc/nginx/sites-available/tomcat.conf
    # nginx -s reload

    [ip: _, port: port] = BambooWeb.Endpoint.config(:http)
    body = EEx.eval_string(
"""
server {
  listen 80;

  server_name _;
  access_log /var/log/nginx/<%= name %>.access.log;
  error_log /var/log/nginx/<%= name %>.error.log;

  location / {
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Server $host;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_pass http://127.0.0.1:<%= port %>/;
  }
}
""", [port: port, name: :bamboo])
    text(conn, body)
  end

  # https://www.atlantic.net/vps-hosting/how-to-setup-tomcat-with-nginx-as-a-reverse-proxy-on-ubuntu/
  def service_conf(conn, _params) do
    # /etc/systemd/system/tomcat.service
    # systemctl daemon-reload
    [ip: _, port: port] = BambooWeb.Endpoint.config(:http)
    body = EEx.eval_string(
"""
[Unit]
Description=A smart router.
After=network.target

[Service]
Type=simple

User=root
Group=root

Environment="PORT=<%= port %>"
Environment="SECRET_KEY_BASE='$(mix phx.gen.secret)'"
Environment="DATABASE_URL='postgresql://www:$(pwgen 32 1)@127.0.0.1:5432/<%= name %>'"

ExecStart=/opt/<%= name %>/app/bin/server

[Install]
WantedBy=multi-user.target
""", [port: port, name: :bamboo]
    )
    text(conn, body)
  end
end
