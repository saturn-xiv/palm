package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers;

import java.util.Locale;

import org.springframework.boot.info.BuildProperties;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.responses.Layout;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.LocaleRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.I18nService;

import jakarta.annotation.Resource;

@Controller("portal.c.home")
public class HomeController {
    @GetMapping("/install")
    public String getInstall(Locale locale, Model model) {
        var lang = i18nService.detect(locale);
        var layout = new Layout(lang, buildProperties, i18nService, localeRepository);
        layout.setTitle("sss");
        model.addAttribute("layout", layout);
        model.addAttribute("title", "ttt");
        return "install";
    }

    @GetMapping("/")
    @PreAuthorize("permitAll()")
    public String index(Model model) {
        // TODO
        model.addAttribute("title", "Home");
        model.addAttribute("lang", "en-US");
        return "home";
    }

    @Resource
    BuildProperties buildProperties;
    @Resource
    I18nService i18nService;
    @Resource
    LocaleRepository localeRepository;
}
