package com.github.saturn_siv.palm.dahlia.config

import com.github.saturn_siv.palm.dahlia.commands.UsersCommand
import org.springframework.context.annotation.Configuration
import org.springframework.shell.command.annotation.EnableCommand

@Configuration
@EnableCommand(UsersCommand::class)
class CommandsConfiguration {
}