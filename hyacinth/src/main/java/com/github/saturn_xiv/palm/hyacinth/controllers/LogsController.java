package com.github.saturn_xiv.palm.hyacinth.controllers;

import jakarta.annotation.Resource;

import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;

import com.github.saturn_xiv.palm.hyacinth.repositories.LogRepository;

@Controller
public class LogsController {
    @GetMapping("/logs")
    public String show(Model model) {
        var items = logRepository.findTimeTakingLagerThan(500);
        // TODO
        model.addAttribute("hi", "Hello, Palm!");
        model.addAttribute("items", items);
        return "logs";
    }

    @Resource
    LogRepository logRepository;
}
