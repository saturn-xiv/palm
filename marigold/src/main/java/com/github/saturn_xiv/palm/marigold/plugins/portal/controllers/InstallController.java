package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers;

import java.time.ZoneId;
import java.util.HashMap;
import java.util.Locale;
import java.util.Map;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.context.MessageSource;
import org.springframework.http.HttpStatus;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.validation.BindingResult;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.ModelAttribute;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.server.ResponseStatusException;
import org.springframework.web.servlet.mvc.support.RedirectAttributes;

import com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.forms.InstallForm;
import com.github.saturn_xiv.palm.marigold.plugins.portal.repositories.UserRepository;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.LayoutService;
import com.github.saturn_xiv.palm.marigold.plugins.portal.services.UserService;

import jakarta.annotation.Resource;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.validation.Valid;

@Controller("portal.c.install")
@RequestMapping("/install")
public class InstallController {
    @PostMapping
    public String postInstall(HttpServletRequest request, RedirectAttributes redirectAttributes,
            @Valid @ModelAttribute(HomeController.FORM) InstallForm form,
            BindingResult result,
            Model model,
            Locale locale) {
        if (userRepository.count() > 0) {
            throw new ResponseStatusException(HttpStatus.FORBIDDEN);
        }

        if (result.hasErrors()) {
            HomeController.setErrors(model, result);
        } else if (!form.getPassword().equals(form.getPasswordConfirmation())) {
            logger.error("passwords don't match");
            Map<String, String> errors = new HashMap<>();
            errors.putIfAbsent("passwordConfirmation",
                    messageSource.getMessage("form.fields.password-confirmation.error", null, locale));
            model.addAttribute(HomeController.ERRORS, errors);
        } else {
            String ip = request.getRemoteHost();
            String email = form.getEmail().trim().toLowerCase();
            String name = form.getName().trim();
            logger.warn("init system operator with {}<{}>", name, email);
            try {
                userService.install(name, email, form.getPassword(),
                        Locale.forLanguageTag(form.getLang()), ZoneId.of(form.getTimezone()), ip);
                HomeController.succeed(redirectAttributes,
                        messageSource.getMessage("pages.install.succeed", null, locale));
                return HomeController.REDIRECT_HOME;
            } catch (Exception e) {
                logger.error("install", e);
                Map<String, String> errors = new HashMap<>();
                errors.putIfAbsent("form", e.getMessage());
                model.addAttribute(HomeController.ERRORS, errors);
            }
        }
        return layout(model, locale);
    }

    @GetMapping
    public String getInstall(Model model, Locale locale) {
        if (userRepository.count() > 0) {
            throw new ResponseStatusException(HttpStatus.FORBIDDEN);
        }
        model.addAttribute(HomeController.FORM, new InstallForm());
        return layout(model, locale);
    }

    private String layout(Model model, Locale locale) {
        HomeController.setModel(model, locale, layoutService, "pages.install.title");
        return "install";
    }

    @Resource
    UserRepository userRepository;
    @Resource
    UserService userService;
    @Resource
    LayoutService layoutService;
    @Resource
    MessageSource messageSource;

    private final static Logger logger = LoggerFactory.getLogger(InstallController.class);
}
