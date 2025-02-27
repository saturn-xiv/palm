package com.github.saturn_siv.palm.dahlia.commands

import jakarta.validation.constraints.Email
import jakarta.validation.constraints.Size
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import org.springframework.shell.command.annotation.Command
import org.springframework.shell.command.annotation.Option


@Command(command = ["users"])
class UsersCommand {
    @Command(command = ["create-by-email"], description = "Create an email user")
    fun createByEmail(
        @Option(shortNames = ['n'], required = true)@Size(min = 2, max = 32) name: String,
        @Option(shortNames = ['e'], required = true)@Size(min = 6, max = 127) @Email email: String,
        @Option(shortNames = ['p'], required = true)@Size(min = 6, max = 32) password: String
    ) {
        logger.warn("create an new user {}<{}>", name, email)
    }

    companion object {
        @JvmStatic
        val logger: Logger = LoggerFactory.getLogger(UsersCommand::class.java);
    }
}