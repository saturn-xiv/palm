package com.github.saturn_siv.palm.dahlia.services

import org.springframework.stereotype.Component
import java.util.*

@Component("dahlia.s.git-properties")
class GitPropertiesService {

    fun buildVersion(): String {
        return this.get("git.build.version")
    }

    fun commitId(): String {
        return this.get("git.commit.id.abbrev")
    }

    private fun get(key: String): String {
        val props = this::class.java.classLoader.getResourceAsStream("git.properties").use {
            Properties().apply { load(it) }
        }
        return props.getProperty(key)
    }
}