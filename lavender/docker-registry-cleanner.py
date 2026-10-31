import argparse
import logging
import sys

import requests

logger = logging.getLogger(__name__)


def list_all_repositories(host):
    logger.debug("fetch all repositories")
    res = requests.get(f"{host}/v2/_catalog")
    res.raise_for_status()
    body = res.json()
    logger.debug("%s", body)
    return body["repositories"]


def list_tags_for_repository(host, repository):
    logger.debug("fetch tags for %s", repository)
    res = requests.get(f"{host}/v2/{repository}/tags/list")
    res.raise_for_status()
    body = res.json()
    logger.debug("%s", body)
    return body["tags"]


def resolve_manifest_digest_for_tag(host, repository, tag):
    logger.debug("fetch manifest digest for %s:%s", repository, tag)
    res = requests.head(
        f"{host}/v2/{repository}/manifests/{tag}",
        headers={"Accept": "application/vnd.docker.distribution.manifest.v2+json"},
    )
    res.raise_for_status()
    # logger.debug("%s", res.headers)
    return res.headers["docker-content-digest"]


def delete_manifest(host, repository, manifest):
    logger.debug("delete %s/%s", repository, manifest)
    res = requests.delete(f"{host}/v2/{repository}/manifests/{manifest}")
    res.raise_for_status()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Docker registry cleanner.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("-d", "--debug", action="store_true", help="run on debug mode")
    parser.add_argument(
        "-H", "--host", default="http://127.0.0.1:5000", help="docker registry host"
    )
    args = parser.parse_args()
    logging.basicConfig(
        format="%(asctime)s %(levelname).1s %(message)s",
        level=logging.DEBUG if args.debug else logging.INFO,
    )
    logger.debug("running on debug mode")
    logger.info("fetch images from %s", args.host)
    repositories = list_all_repositories(args.host)
    for repository in repositories:
        tags = list_tags_for_repository(args.host, repository)
        if tags:
            for tag in tags:
                manifest_digest = resolve_manifest_digest_for_tag(
                    args.host, repository, tag
                )
                logger.debug("docker-content-digest: %s", manifest_digest)
                while True:
                    reply = (
                        input(f"Delete the manifest {repository}:{tag}(y/n/q): ")
                        .strip()
                        .lower()
                    )
                    if reply in ["yes", "y"]:
                        delete_manifest(args.host, repository, manifest_digest)
                        break
                    elif reply in ["no", "n"]:
                        break
                    elif reply in ["quit", "q"]:
                        logger.info("exit.")
                        sys.exit()
                    else:
                        logger.error("Invalid input, please enter 'y' or 'n'.")

    logger.info(
        "run `/bin/registry garbage-collect /etc/docker/registry/config.yml` if you need disk space back"
    )
