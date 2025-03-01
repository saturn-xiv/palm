from flask import render_template


def home():
    # TODO
    return render_template('home.html')


def rss_xml(lang):
    # TODO
    return "rxx.xml"


def sitemap_xml_by_lang(lang):
    # TODO
    return "sitemap.xml"


def sitemap_xml():
    # TODO
    return "sitemap.xml"


def robots_txt():
    # TODO
    return "robots.txt"


def show_page_by_slug(slug):
    # TODO
    return "robots.txt"

def not_found(e):
    return render_template("not-found.html"),404

def internal_server(e):
    return render_template("internal-server.html"),500
