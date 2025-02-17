package com.github.saturn_siv.palm.dahlia.plugins.cms.controllers

import org.springframework.stereotype.Controller
import org.springframework.ui.Model
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PostMapping
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.servlet.ModelAndView

@Controller("dahlia.cms.c.users-sign-in")
@RequestMapping("/users")
class UsersSignInController {
    @GetMapping("/sign-in")
    fun get(model: Model): ModelAndView {
//    TODO
        val view = ModelAndView("bootstrap/users/sign-in")
        view.addObject("title", "Users sign in");
        return view;
    }

    @PostMapping("/sign-in")
    fun post(model: Model): String {
//    TODO
        return "todo";
    }
}