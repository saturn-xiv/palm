package com.github.saturn_siv.palm.dahlia.controllers

import org.springframework.security.access.prepost.PreAuthorize
import org.springframework.stereotype.Controller
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.servlet.ModelAndView

@Controller("dahlia.c.home")
class HomeController {

    @PreAuthorize("permitAll()")
    @GetMapping("/")
    fun home(): ModelAndView {
//    TODO
        val view = ModelAndView("bootstrap/home")
        view.addObject("title", "Home");
        return view;
    }
//  TODO  robots.txt
}