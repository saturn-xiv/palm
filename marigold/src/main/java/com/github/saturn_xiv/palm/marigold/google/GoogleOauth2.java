package com.github.saturn_xiv.palm.marigold.google;

import java.io.IOException;
import java.io.Serializable;
import java.util.Arrays;
import java.util.List;

import org.apache.commons.lang3.tuple.Pair;

import com.google.api.client.auth.oauth2.AuthorizationCodeFlow;
import com.google.api.client.auth.oauth2.BearerToken;
import com.google.api.client.auth.oauth2.BrowserClientRequestUrl;
import com.google.api.client.auth.oauth2.ClientParametersAuthentication;
import com.google.api.client.auth.oauth2.Credential;
import com.google.api.client.auth.oauth2.TokenResponse;
import com.google.api.client.http.GenericUrl;
import com.google.api.client.http.javanet.NetHttpTransport;
import com.google.api.client.json.gson.GsonFactory;
import com.google.api.client.util.store.MemoryDataStoreFactory;

public record GoogleOauth2(String clientId, String clientSecret) implements Serializable {
    public String signInUrlForBrowser(String redirectUri, String state) {
        return new BrowserClientRequestUrl(AUTHORIZATION_SERVER_URL, this.clientId).setState(state).setScopes(SCOPES)
                .setRedirectUri(redirectUri).build();
    }

    public Pair<TokenResponse, Credential> exchange(String code, String redirectUri, String user) throws IOException {
        var flow = this.buildFlow();
        var tokenResponse = flow.newTokenRequest(code)
                .setRedirectUri(redirectUri)
                .execute();
        var credential = flow.createAndStoreCredential(tokenResponse, user);
        return Pair.of(tokenResponse, credential);
    }

    public String signInUrl(String redirectUri, String state) throws IOException {
        return buildFlow()
                .newAuthorizationUrl()
                .setRedirectUri(redirectUri)
                .setState(state)
                .build();
    }

    private AuthorizationCodeFlow buildFlow() throws IOException {
        return new AuthorizationCodeFlow.Builder(
                BearerToken.authorizationHeaderAccessMethod(),
                new NetHttpTransport(),
                GsonFactory.getDefaultInstance(),
                new GenericUrl("https://googleapis.com"),
                new ClientParametersAuthentication(this.clientId, this.clientSecret),
                this.clientId, AUTHORIZATION_SERVER_URL)
                .setScopes(SCOPES)
                .setDataStoreFactory(new MemoryDataStoreFactory())
                .build();
    }

    private final static String AUTHORIZATION_SERVER_URL = "https://google.com";
    // https://developers.google.com/identity/protocols/oauth2/scopes
    private final static List<String> SCOPES = Arrays.asList("https://www.googleapis.com/auth/userinfo.email",
            "https://www.googleapis.com/auth/userinfo.profile", "openid");
}
