package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;
import java.security.GeneralSecurityException;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.Locale;
import java.util.NoSuchElementException;
import java.util.Optional;

import org.springframework.boot.info.BuildProperties;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.I18nService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.SettingService;

public class Layout implements Serializable {
    public Layout(Locale locale, BuildProperties buildProperties, I18nService i18n, SettingService settings,
            LocaleRepository localeRepository) {

        this.locale = locale.toLanguageTag();
        this.title = i18n.t(locale, TITLE);
        this.subhead = i18n.t(locale, SUBHEAD);
        this.description = i18n.t(locale, DESCRIPTION);
        this.copyright = i18n.t(locale, COPYRIGHT);
        this.languages = localeRepository.findDistinctLanguages();

        this.project = new Project(buildProperties.getName(), "https://github.com/saturn-xiv/palm",
                buildProperties.getVersion());
        this.buildTime = buildProperties.getTime();
        try {
            this.author = settings.get(AUTHOR, Author.class);
        } catch (NoSuchElementException | GeneralSecurityException e) {
            this.author = new Author("", "");
        }
        try {

            this.keywords = settings.getList(KEYWORDS, String.class);
        } catch (NoSuchElementException | GeneralSecurityException e) {
            this.keywords = new ArrayList<>();
        }
    }

    public Layout() {
        this.keywords = new ArrayList<>();
        this.languages = new ArrayList<>();
    }

    public static final String TITLE = "site.title";
    public static final String SUBHEAD = "site.subhead";
    public static final String DESCRIPTION = "site.description";
    public static final String COPYRIGHT = "site.copyright";
    public static final String AUTHOR = "site.author";
    public static final String KEYWORDS = "site.author";

    public record Author(String name, String email) implements Serializable {
    }

    public record Project(String name, String url, String version) {
    }

    private Optional<String> favicon;
    private String title;
    private String subhead;
    private List<String> keywords;
    private Author author;
    private Project project;
    private String description;
    private String copyright;
    private String locale;
    private List<String> languages;
    private Instant buildTime;

    public Optional<String> getFavicon() {
        return favicon;
    }

    public void setFavicon(Optional<String> favicon) {
        this.favicon = favicon;
    }

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public String getSubhead() {
        return subhead;
    }

    public void setSubhead(String subhead) {
        this.subhead = subhead;
    }

    public List<String> getKeywords() {
        return keywords;
    }

    public void setKeywords(List<String> keywords) {
        this.keywords = keywords;
    }

    public Author getAuthor() {
        return author;
    }

    public void setAuthor(Author author) {
        this.author = author;
    }

    public Project getProject() {
        return project;
    }

    public void setProject(Project project) {
        this.project = project;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public String getCopyright() {
        return copyright;
    }

    public void setCopyright(String copyright) {
        this.copyright = copyright;
    }

    public String getLocale() {
        return locale;
    }

    public void setLocale(String locale) {
        this.locale = locale;
    }

    public List<String> getLanguages() {
        return languages;
    }

    public void setLanguages(List<String> languages) {
        this.languages = languages;
    }

    public Instant getBuildTime() {
        return buildTime;
    }

    public void setBuildTime(Instant buildTime) {
        this.buildTime = buildTime;
    }

}
