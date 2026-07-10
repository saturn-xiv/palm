# RHODODENDRON

## Usage

```bash
git clone -b phoenix https://github.com/saturn-xiv/palm.git $HOME/workspace/palm
cd $HOME/workspace/palm/
./scripts/build.sh
```

To start your Phoenix server:

- Run `mix setup` to install and setup dependencies
- Start Phoenix endpoint with `mix phx.server` or inside IEx with `iex -S mix phx.server`

Now you can visit [`localhost:4000`](http://localhost:4000) from your browser.

Ready to run in production? Please [check our deployment guides](https://phoenix.hexdocs.pm/deployment.html).

## Development

### Database prepare

```sql
CREATE USER www WITH PASSWORD 'change-me';
CREATE DATABASE rhododendron_dev WITH OWNER www ENCODING='UTF8';
```

```bash
mix ecto.migrate
mix ecto.migrations
mix ecto.rollback
```

## Testing

```bash
mix test test/rhododendron/crypto_test.exs
```

## Learn more

- [Erl_Interface User's Guide](https://www.erlang.org/doc/apps/erl_interface/ei_users_guide.html)
- [ISO 4217 Currency codes](https://www.iso.org/iso-4217-currency-codes.html)
- [Install WSL](https://docs.microsoft.com/en-us/windows/wsl/install)
- [Keep a changelog](https://keepachangelog.com/en/1.0.0/)
- [Semantic Versioning 2.0.0](https://semver.org/)
- [What's New in Globalization and Localization](<https://docs.microsoft.com/en-us/previous-versions/dotnet/netframework-4.0/dd997383(v=vs.100)?redirectedfrom=MSDN>)
- [ISO 4217 CURRENCY CODES](https://www.iso.org/iso-4217-currency-codes.html)
- [BCP 47](https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry)
- [National Address Database (NAD) Disclaimer](https://www.transportation.gov/gis/national-address-database/national-address-database-nad-disclaimer)
- [Web Tools API Portal](https://www.usps.com/business/web-tools-apis/)
- [Reviewing address formats](https://help.shopify.com/en/manual/shipping/shopify-shipping/reviewing-address-formats)
- [handlebars](https://handlebarsjs.com/guide/)
- [Versioning gRPC services](https://docs.microsoft.com/en-us/aspnet/core/grpc/versioning?view=aspnetcore-6.0)
- [Modified Preorder Tree Traversal](https://gist.github.com/tmilos/f2f999b5839e2d42d751)
- [Media Types](https://www.iana.org/assignments/media-types/media-types.xhtml)
- [Protobuf Version Support](https://protobuf.dev/support/version-support/)

### UI

- [MUI](https://mui.com/getting-started/installation/)
- [Google Fonts](https://developers.google.com/fonts/docs/material_icons#setup_method_2_self_hosting)

### Token

- [Sign in with App Passwords](https://support.google.com/accounts/answer/185833?hl=en)
- [Casbin is a powerful and efficient open-source access control library](https://casbin.org/docs/en/overview)
- [The RBAC96 Model](https://profsandhu.com/cs6393_s12/lecture-rbac96.pdf)
- [Password Hashing Competition](https://www.password-hashing.net/)
- [Security DO's and DON'Ts](https://quasar.dev/security/dos-and-donts)
- [Tinkey overview](https://developers.google.com/tink/tinkey-overview)
