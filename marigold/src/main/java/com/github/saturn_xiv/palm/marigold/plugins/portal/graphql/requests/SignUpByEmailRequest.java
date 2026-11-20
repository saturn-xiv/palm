package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.requests;

import java.io.Serializable;

import jakarta.validation.constraints.Email;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

public record SignUpByEmailRequest(@NotBlank @Size(min = 2, max = 31) String name,
                @NotBlank @Email @Size(min = 5, max = 63) String email,
                @NotBlank @Size(min = 6) String password,
                @NotBlank @Size(min = 2, max = 15) String lang, @NotBlank @Size(min = 2, max = 31) String timezone,
                @NotBlank String home)
                implements Serializable {

}
