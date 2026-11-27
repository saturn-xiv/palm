package com.github.saturn_xiv.palm.marigold.plugins.portal.models.user;

import java.time.Instant;
import java.util.ArrayList;
import java.util.List;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.annotations.SQLDelete;
import org.hibernate.annotations.SQLRestriction;
import org.hibernate.type.SqlTypes;
import org.springframework.data.annotation.CreatedDate;
import org.springframework.data.annotation.LastModifiedDate;
import org.springframework.data.jpa.domain.support.AuditingEntityListener;

import com.github.saturn_xiv.palm.marigold.wechat.responses.AccessTokenByCodeResponse;
import com.github.saturn_xiv.palm.marigold.wechat.responses.UserInfoResponse;

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
@Table(name = "wechat_oauth2_users")
@SQLDelete(sql = "UPDATE wechat_oauth2_users SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
@SQLRestriction("deleted_at IS NULL")
public class ByWechatOauth2 implements Info {
    private static final long serialVersionUID = 1L;

    public ByWechatOauth2() {
        this.privilege = new ArrayList<>();
    }

    public void merge(AccessTokenByCodeResponse accessTokenByCode, UserInfoResponse info) {
        this.accessToken = accessTokenByCode.getAccessToken();
        this.refreshToken = accessTokenByCode.getRefreshToken();
        this.privilege = info.getPrivilege();
        this.openId = info.getOpenid();
        this.unionId = info.getUnionId();
        this.nickname = info.getNickname();
        this.headImgUrl = info.getHeadImgUrl();
    }

    @Override
    public String label() {
        return String.format("%s.%s", appId, openId);
    }

    @Override
    public boolean isAvailable() {
        return this.deletedAt == null && this.lockedAt == null
                && this.user.isAvailable();
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
    @Column(nullable = false, updatable = false)
    @Size(max = 63)
    private String unionId;
    @Column(nullable = false, updatable = false)
    @Size(max = 63)
    private String appId;
    @Column(nullable = false, updatable = false)
    @Size(max = 63)
    private String openId;
    @Column(nullable = false)
    @Size(max = 31)
    private String nickname;
    @Size(min = 9, max = 127)
    private String headImgUrl;
    @JdbcTypeCode(SqlTypes.JSON)
    @Column(columnDefinition = "jsonb", nullable = false)
    private List<String> privilege;
    @Column(nullable = false)
    @Size(max = 63)
    private String accessToken;
    @Column(nullable = false)
    @Size(max = 63)
    private String refreshToken;
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

    public String getUnionId() {
        return unionId;
    }

    public void setUnionId(String unionId) {
        this.unionId = unionId;
    }

    public String getAppId() {
        return appId;
    }

    public void setAppId(String appId) {
        this.appId = appId;
    }

    public String getOpenId() {
        return openId;
    }

    public void setOpenId(String openId) {
        this.openId = openId;
    }

    public String getNickname() {
        return nickname;
    }

    public void setNickname(String nickname) {
        this.nickname = nickname;
    }

    public String getHeadImgUrl() {
        return headImgUrl;
    }

    public void setHeadImgUrl(String headImgUrl) {
        this.headImgUrl = headImgUrl;
    }

    public List<String> getPrivilege() {
        return privilege;
    }

    public void setPrivilege(List<String> privilege) {
        this.privilege = privilege;
    }

    public String getAccessToken() {
        return accessToken;
    }

    public void setAccessToken(String accessToken) {
        this.accessToken = accessToken;
    }

    public String getRefreshToken() {
        return refreshToken;
    }

    public void setRefreshToken(String refreshToken) {
        this.refreshToken = refreshToken;
    }

    public int getVersion() {
        return version;
    }

    public void setVersion(int version) {
        this.version = version;
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
