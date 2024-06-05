defmodule TuberoseWeb.Etc.NginxConfController do
  use TuberoseWeb, :controller

  def get(conn, %{"domain" => domain, "port" => port}) do
    text(conn, """
    # /etc/nginx/sites-enabled/#{domain}.conf
    server {
      listen 80;

      server_name #{domain};
      access_log /var/log/nginx/#{domain}.access.log;
      error_log  /var/log/nginx/#{domain}.error.log;

      gzip on;
      gzip_comp_level 9;
      gzip_min_length 1k;
      gzip_types text/plain text/css application/xml application/javascript;
      gzip_vary on;
      client_max_body_size 128M;

      location /my/ {
        alias /var/www/#{domain}/dashboard/;
        try_files $uri $uri/ /my/index.html;

        location ~* \.(css|js|png|jpg|jpeg|gif|gz|svg|mp4|ogg|ogv|webm|htc|xml|woff)$ {
          access_log off;
          expires max;
        }
      }

      location / {
        proxy_set_header X-Forwarded-Proto http;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Host $http_host;
        proxy_redirect off;
        proxy_pass http://127.0.0.1:#{port};
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
      }
    }
    """)
  end
end
