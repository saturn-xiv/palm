package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers;

import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.info.BuildProperties;
import org.springframework.context.MessageSource;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.validation.BindingResult;
import org.springframework.validation.FieldError;
import org.springframework.web.bind.annotation.GetMapping;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Layout;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.I18nService;

import jakarta.annotation.Resource;

@Controller("portal.c.home")
public class HomeController {

    public static void setErrors(Model model, BindingResult result) {
        Map<String, String> errors = new HashMap<>();
        for (FieldError error : result.getFieldErrors()) {
            logger.error("{} {}", error.getField(), error.getDefaultMessage());
            errors.putIfAbsent(error.getField(), error.getDefaultMessage());
        }
        model.addAttribute(ERRORS, errors);
    }

    @GetMapping("/term-and-conditions")
    public String termAndConditions(Model model, Locale locale) {
        setModel(model, locale, buildProperties, i18nService, localeRepository, "pages.term-and-conditions.title");
        return "term-and-conditions";
    }

    @GetMapping("/")
    @PreAuthorize("permitAll()")
    public String index(Model model) {
        // TODO
        model.addAttribute("title", "Home");
        model.addAttribute("lang", "en-US");
        return "home";
    }

    public static void setModel(Model model, Locale locale, BuildProperties buildProperties, I18nService i18nService,
            LocaleRepository localeRepository, String title) {
        var layout = new Layout(locale, buildProperties, i18nService, localeRepository);
        model.addAttribute("layout", layout);
        model.addAttribute("title", title);
    }

    @Resource
    I18nService i18nService;
    @Resource
    LocaleRepository localeRepository;
    @Resource
    MessageSource messageSource;
    @Resource
    BuildProperties buildProperties;

    public final static String FORM = "form";
    public final static String ERRORS = "errors";
    private final static Logger logger = LoggerFactory.getLogger(InstallController.class);
}
