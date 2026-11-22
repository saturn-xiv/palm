package com.github.saturn_xiv.palm.marigold.plugins.portal.models.user;

import java.io.Serializable;
import java.time.Instant;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.annotations.SQLDelete;
import org.hibernate.annotations.SQLRestriction;
import org.hibernate.type.SqlTypes;
import org.springframework.data.annotation.CreatedDate;
import org.springframework.data.annotation.LastModifiedDate;
import org.springframework.data.jpa.domain.support.AuditingEntityListener;

import com.google.api.client.auth.oauth2.TokenResponse;
import com.google.api.services.oauth2.model.Userinfo;
import com.google.auth.oauth2.AccessToken;
import com.google.auth.oauth2.GoogleCredentials;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.EntityListeners;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.Table;
import jakarta.persistence.Version;
import jakarta.validation.constraints.Size;

@Entity
@EntityListeners(AuditingEntityListener.class)
@Table(name = "google_oauth2_users")
@SQLDelete(sql = "UPDATE google_oauth2_users SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
@SQLRestriction("deleted_at IS NULL")
public class ByGoogleOauth2 implements Serializable {
    private static final long serialVersionUID = 1L;

    public void merge(TokenResponse tokenResponse, Userinfo info) {
        this.token = tokenResponse;

        this.sub = info.getId();
        this.email = info.getEmail();
        this.emailVerified = info.isVerifiedEmail();
        this.familyName = info.getFamilyName();
        this.givenName = info.getGivenName();
        this.gender = info.getGender();
        this.hostedDomain = info.getHd();
        this.name = info.getName();
        this.link = info.getLink();
        this.picture = info.getPicture();
        this.locale = info.getLocale();
    }

    public String id() {
        return String.format("%s.%s", clientId, email);
    }

    public boolean isAvailable() {
        return this.deletedAt == null && this.lockedAt == null
                && this.user.isAvailable();
    }

    public GoogleCredentials credential() {
        return GoogleCredentials.create(new AccessToken(this.token.getAccessToken(), null));
    }

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;
    @ManyToOne
    @JoinColumn(name = "user_id", nullable = false)
    private Item user;
    @Column(nullable = false)
    @Size(min = 36, max = 36)
    private String sn;
    @Column(nullable = false)
    @Size(max = 63)
    private String clientId;
    @Column(nullable = false)
    @Size(max = 36)
    private String sub;
    @JdbcTypeCode(SqlTypes.JSON)
    @Column(columnDefinition = "jsonb", nullable = false)
    TokenResponse token;
    @Column(nullable = false)
    @Size(min = 5, max = 127)
    private String email;
    private boolean emailVerified;
    @Size(min = 5, max = 63)
    private String hostedDomain;
    @Size(max = 63)
    private String givenName;
    @Size(max = 63)
    private String gender;
    @Size(min = 9, max = 127)
    private String link;
    @Size(max = 63)
    private String familyName;
    @Size(max = 63)
    private String name;
    @Size(min = 9, max = 127)
    private String picture;
    @Size(min = 2, max = 15)
    private String locale;
    private Instant lockedAt;
    private Instant deletedAt;
    @Version
    private int version;
    @LastModifiedDate
    @Column(nullable = false)
    private Instant updatedAt;
    @CreatedDate
    @Column(nullable = false, updatable = false)
    private Instant createdAt;

    public Integer getId() {
        return id;
    }

    public void setId(Integer id) {
        this.id = id;
    }

    public Item getUser() {
        return user;
    }

    public void setUser(Item user) {
        this.user = user;
    }

    public String getSn() {
        return sn;
    }

    public void setSn(String sn) {
        this.sn = sn;
    }

    public String getClientId() {
        return clientId;
    }

    public void setClientId(String clientId) {
        this.clientId = clientId;
    }

    public String getSub() {
        return sub;
    }

    public void setSub(String sub) {
        this.sub = sub;
    }

    public String getEmail() {
        return email;
    }

    public void setEmail(String email) {
        this.email = email;
    }

    public boolean isEmailVerified() {
        return emailVerified;
    }

    public void setEmailVerified(boolean emailVerified) {
        this.emailVerified = emailVerified;
    }

    public String getHostedDomain() {
        return hostedDomain;
    }

    public void setHostedDomain(String hostedDomain) {
        this.hostedDomain = hostedDomain;
    }

    public String getGivenName() {
        return givenName;
    }

    public void setGivenName(String givenName) {
        this.givenName = givenName;
    }

    public String getGender() {
        return gender;
    }

    public void setGender(String gender) {
        this.gender = gender;
    }

    public String getLink() {
        return link;
    }

    public void setLink(String link) {
        this.link = link;
    }

    public String getFamilyName() {
        return familyName;
    }

    public void setFamilyName(String familyName) {
        this.familyName = familyName;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getPicture() {
        return picture;
    }

    public void setPicture(String picture) {
        this.picture = picture;
    }

    public String getLocale() {
        return locale;
    }

    public void setLocale(String locale) {
        this.locale = locale;
    }

    public TokenResponse getToken() {
        return token;
    }

    public void setToken(TokenResponse token) {
        this.token = token;
    }

    public Instant getLockedAt() {
        return lockedAt;
    }

    public void setLockedAt(Instant lockedAt) {
        this.lockedAt = lockedAt;
    }

    public Instant getDeletedAt() {
        return deletedAt;
    }

    public void setDeletedAt(Instant deletedAt) {
        this.deletedAt = deletedAt;
    }

    public int getVersion() {
        return version;
    }

    public void setVersion(int version) {
        this.version = version;
    }

    public Instant getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(Instant updatedAt) {
        this.updatedAt = updatedAt;
    }

    public Instant getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Instant createdAt) {
        this.createdAt = createdAt;
    }

}
