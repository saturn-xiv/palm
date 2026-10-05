from opensearchpy import OpenSearch


class Node:
    def __init__(self, host, port=9200, tls=False):
        self.client = OpenSearch(
            [{'host': host, 'port': port}], http_compress=True, use_ssl=tls)

    # curl -X GET "http://localhost:9200/_cluster/health?pretty"
    def ping(self, db):
        res = self.client.cluster.health()
