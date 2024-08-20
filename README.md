# PALM

A total free education &amp; translation solution.

## Usage

```bash
$ git clone https://github.com/saturn-xiv/palm.git ~/workspace/palm
$ cd ~/workspace/palm
$ git submodule update --init --recursive
$ podman run --rm -it --events-backend=file --network host -v $PWD:/workspace:z ubuntu:latest
> cd /workspace
> ./scripts/loquat.sh
```

## Projects

- `loquat` encrypt thrift-rpc service(by Google Tink)
- `petunia` protocol definition files
- `gourd` cpp protocols of thrift
- `lemon` cpp protocols of grpc
- `coconut` backup tools
- `tuberose` cpp application

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
- [Protobuf Version Support](https://protobuf.dev/support/version-support/)

### Minio

- [Deploy MinIO: Multi-Node Multi-Drive](https://min.io/docs/minio/linux/operations/install-deploy-manage/deploy-minio-multi-node-multi-drive.html)
- [Erasure Code Calculator](https://min.io/product/erasure-code-calculator)

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
