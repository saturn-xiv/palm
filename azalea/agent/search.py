import tomllib

from opensearchpy import OpenSearch


def open_client(config_file):
    with open(config_file, "rb") as buf:
        tmp = tomllib.load(buf)
        config = tmp['opensearch']
        return OpenSearch(
            hosts=[{'host': config['host'], 'port': config['port']}],
            http_compress=True,
            http_auth=(config['user'], config['password']),
            use_ssl=False,
        )
