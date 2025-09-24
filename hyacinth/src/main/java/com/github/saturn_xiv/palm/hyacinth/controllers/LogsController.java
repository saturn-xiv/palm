package com.github.saturn_xiv.palm.hyacinth.controllers;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

@Controller
public class LogsController {
    @GetMapping("/logs")
    public String logs(Model model) {
        // TODO
        model.addAttribute("hi", "Hello, Palm!");
        return "logs";
    }
}
