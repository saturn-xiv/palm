package com.github.saturn_xiv.palm.marigold.plugins.portal.controllers.forms;

import java.io.Serializable;

import jakarta.validation.constraints.AssertTrue;
import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.NotEmpty;
import jakarta.validation.constraints.Size;

public class InstallForm implements Serializable {
    public InstallForm() {
        this.name = "";
        this.email = "";
    }

    @NotEmpty
    @NotBlank
    @Size(min = 2, max = 31)
    private String name;
    @NotEmpty
    @Email
    @Size(min = 5, max = 63)
    private String email;
    @NotEmpty
    @Size(min = 6, max = 31)
    private String password;
    @NotEmpty
    private String passwordConfirmation;
    @NotEmpty
    @Size(min = 2, max = 15)
    private String lang;
    @NotEmpty
    @Size(min = 1, max = 31)
    private String timezone;
    @AssertTrue
    private boolean agree;

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public String getPassword() {
        return password;
    }

    public void setPassword(String password) {
        this.password = password;
    }

    public String getPasswordConfirmation() {
        return passwordConfirmation;
    }

    public void setPasswordConfirmation(String passwordConfirmation) {
        this.passwordConfirmation = passwordConfirmation;
    }

    public String getLang() {
        return lang;
    }

    public void setLang(String lang) {
        this.lang = lang;
    }

    public String getTimezone() {
        return timezone;
    }

    public void setTimezone(String timezone) {
        this.timezone = timezone;
    }

    public boolean isAgree() {
        return agree;
    }

    public void setAgree(boolean agree) {
        this.agree = agree;
    }

}
