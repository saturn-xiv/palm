package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers;

import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.context.MessageSource;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.validation.BindingResult;
import org.springframework.validation.FieldError;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.servlet.mvc.support.RedirectAttributes;

import com.github.saturn_xiv.palm.marigold.plugins.portal.services.LayoutService;

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
        setModel(model, locale, layoutService, "pages.term-and-conditions.title");
        return "term-and-conditions";
    }

    @GetMapping("/")
    @PreAuthorize("permitAll()")
    public String index(Model model, Locale locale) {
        setModel(model, locale, layoutService, "pages.home.title");
        return "home";
    }

    public static void setModel(Model model, Locale locale, LayoutService layoutService, String title) {
        var layout = layoutService.fetch(locale);

        model.addAttribute("layout", layout);
        model.addAttribute("title", title);
    }

    public static void succeed(RedirectAttributes redirectAttributes, String message) {
        redirectAttributes.addFlashAttribute(SUCCEED_FLASH, message);
    }

    public static void failed(RedirectAttributes redirectAttributes, String message) {
        redirectAttributes.addFlashAttribute(FAILED_FLASH, message);
    }

    @Resource
    LayoutService layoutService;
    @Resource
    MessageSource messageSource;

    public final static String FORM = "form";
    public final static String ERRORS = "errors";
    public final static String REDIRECT_HOME = "redirect:/";

    private final static String SUCCEED_FLASH = "flashSucceedMessage";
    private final static String FAILED_FLASH = "flashFailedMessage";

    private final static Logger logger = LoggerFactory.getLogger(InstallController.class);
}
