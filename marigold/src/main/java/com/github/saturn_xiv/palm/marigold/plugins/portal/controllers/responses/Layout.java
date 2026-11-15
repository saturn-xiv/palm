package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses;

import java.io.Serializable;
import java.util.List;
import java.util.Locale;

import org.springframework.boot.info.BuildProperties;

import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.I18nService;

public class Layout implements Serializable {
    private static final long serialVersionUID = 1L;

    public static final String TITLE = "site.title";
    public static final String SUBHEAD = "site.subhead";
    public static final String DESCRIPTION = "site.description";
    public static final String COPYRIGHT = "site.copyright";
    public static final String AUTHOR = "site.author";

    public Layout(Locale locale, BuildProperties buildProperties, I18nService i18n, LocaleRepository localeRepository) {
        // TODO
        this.timezone = "UTC";

        this.locale = locale.toLanguageTag();
        this.title = i18n.t(locale, TITLE);
        this.subhead = i18n.t(locale, SUBHEAD);
        this.description = i18n.t(locale, DESCRIPTION);
        this.copyright = i18n.t(locale, COPYRIGHT);
        this.languages = localeRepository.findDistinctLanguages();

        this.project = new Project("Palm", "https://github.com/saturn-xiv/palm", buildProperties.getVersion());
    }

    public record Project(String name, String url, String version) {
    }

    private String locale;
    private String timezone;
    private String title;
    private String subhead;
    private String copyright;
    private String description;
    private Project project;
    private List<String> languages;

    public List<String> getLanguages() {
        return languages;
    }

    public void setLanguages(List<String> languages) {
        this.languages = languages;
    }

    public String getLocale() {
        return locale;
    }

    public void setLocale(String locale) {
        this.locale = locale;
    }

    public String getTitle() {
        return title;
    }

    public Project getProject() {
        return project;
    }

    public void setProject(Project project) {
        this.project = project;
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

    public String getCopyright() {
        return copyright;
    }

    public void setCopyright(String copyright) {
        this.copyright = copyright;
    }

    public String getDescription() {
        return description;
    }

    public void setDescription(String description) {
        this.description = description;
    }

    public String getTimezone() {
        return timezone;
    }

    public void setTimezone(String timezone) {
        this.timezone = timezone;
    }

}
