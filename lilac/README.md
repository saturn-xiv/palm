# LILAC

## Usage

```bash
grpcurl -plaintext localhost:10001 list
grpcurl -plaintext localhost:10001 list palm.lilac.auth.v1.Locale
grpcurl -plaintext -d '{"lang": "en-US"}' localhost:10001 palm.lilac.auth.v1.Locale/ByLang
```
