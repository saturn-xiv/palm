package com.github.saturn_xiv.palm.marigold.controllers;

import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController("c.home")
public class HomeController {

    @GetMapping("/")
    @PreAuthorize("permitAll()")
    public String index(Model model) {
        // TODO
        model.addAttribute("title", "Home");
        model.addAttribute("lang", "en-US");
        return "index";
    }
}
