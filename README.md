# PALM

A total free education & translation solution.

## Usage

- Clone source codes

  ```bash
  # clone source code
  $ git clone --recurse-submodules https://github.com/saturn-xiv/palm.git
  ```

- install dejavu fonts

  ```bash
  # for centos
  sudo yum install dejavu-sans-fonts snmp
  # for archlinux
  sudo pacman -S ttf-dejavu ttf-hanazono ttf-hannom \
    snmp
  # for ubuntu
  sudo apt install fonts-dejavu snmp
  # for alpine
  sudo apk add ttf-dejavu snmp
  ```

- Generate a random secret key: `openssl rand -base64 32`
- Generate a random password: `pwgen 32 1`
- Start casbin-server `CONNECTION_CONFIG_PATH=./casbin/config.json ./bin/\$(uname -p)/bin/casbin -port 8080
`

## Documents

- [Install WSL](https://docs.microsoft.com/en-us/windows/wsl/install)
- [keep a changelog](https://keepachangelog.com/en/1.0.0/)
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

### UI

- [Fluent UI](https://developer.microsoft.com/en-us/fluentui#/controls/web)
- [MUI](https://mui.com/getting-started/installation/)
- [Google Fonts](https://developers.google.com/fonts/docs/material_icons#setup_method_2_self_hosting)

### Token

- [Sign in with App Passwords](https://support.google.com/accounts/answer/185833?hl=en)
- [Casbin is a powerful and efficient open-source access control library](https://casbin.org/docs/en/overview)
- [The RBAC96 Model](https://profsandhu.com/cs6393_s12/lecture-rbac96.pdf)
- [Password Hashing Competition](https://www.password-hashing.net/)
- [Security DO's and DON'Ts](https://quasar.dev/security/dos-and-donts)
- [Tinkey overview](https://developers.google.com/tink/tinkey-overview)

## Sub-projects

- ~~lily~~: tex => pdf/word, epub, Excel, ImageMagic services
- morus: markdown => html services
- musa: WechatPay services
- lilac: casbin/minio/tink services
- camelia: cms/forum services
- ~~daffodil~~: tex, pdf, word, epub, excel, imagemagic services
