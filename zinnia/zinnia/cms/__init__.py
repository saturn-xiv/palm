from flask import render_template

from .. import HTTP_HEADER_CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN_UTF8, CONTENT_TYPE_APPLICATION_XML,  HTTP_STATUS_OK, HTTP_STATUS_NOT_FOUND, HTTP_STATUS_INTERNAL_SERVER_ERROR


def home():
    # TODO
    return render_template('home.html')


def rss_xml(lang):
    # TODO
    return "rss.xml", HTTP_STATUS_OK, {HTTP_HEADER_CONTENT_TYPE: CONTENT_TYPE_APPLICATION_XML}


def sitemap_xml_by_lang(lang):
    # TODO
    return "sitemap.xml", HTTP_STATUS_OK, {HTTP_HEADER_CONTENT_TYPE: CONTENT_TYPE_APPLICATION_XML}


def sitemap_xml():
    # TODO
    return "sitemap.xml", HTTP_STATUS_OK, {HTTP_HEADER_CONTENT_TYPE: CONTENT_TYPE_APPLICATION_XML}


def robots_txt():
    # TODO
    return "robots.txt", HTTP_STATUS_OK, {HTTP_HEADER_CONTENT_TYPE: CONTENT_TYPE_TEXT_PLAIN_UTF8}


def nginx_conf():
    # TODO
    return render_template("nginx.conf"), HTTP_STATUS_OK, {HTTP_HEADER_CONTENT_TYPE: CONTENT_TYPE_TEXT_PLAIN_UTF8}


def show_page_by_slug(slug):
    # TODO
    return "robots.txt"


def not_found(e):
    return render_template("not-found.html"), HTTP_STATUS_NOT_FOUND


def internal_server(e):
    return render_template("internal-server.html"), HTTP_STATUS_INTERNAL_SERVER_ERROR
