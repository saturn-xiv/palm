package com.github.saturn_xiv.palm.marigold.plugins.portal.models;

import java.io.Serializable;
import java.util.Date;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.type.SqlTypes;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.Table;
import jakarta.validation.constraints.Size;

@Entity
@Table(name = "user_bans")
public class Ban implements Serializable {
    private static final long serialVersionUID = 1L;

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;
    @ManyToOne
    @JoinColumn(name = "user_id", nullable = false)
    private Item user;
    @ManyToOne
    @JoinColumn(name = "creator_id", nullable = false)
    private Item creator;
    @Column(nullable = false)
    @Size(min = 7, max = 45)
    private String ip;
    @JdbcTypeCode(SqlTypes.JSON)
    @Column(columnDefinition = "jsonb", nullable = false)
    private MediaContent reason;
    @Column(nullable = false)
    private Date expiresAt;
    private Date deletedAt;
    @Column(nullable = false)
    private Date createdAt;

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

    public Item getCreator() {
        return creator;
    }

    public void setCreator(Item creator) {
        this.creator = creator;
    }

    public String getIp() {
        return ip;
    }

    public void setIp(String ip) {
        this.ip = ip;
    }

    public MediaContent getReason() {
        return reason;
    }

    public void setReason(MediaContent reason) {
        this.reason = reason;
    }

    public Date getExpiresAt() {
        return expiresAt;
    }

    public void setExpiresAt(Date expiresAt) {
        this.expiresAt = expiresAt;
    }

    public Date getDeletedAt() {
        return deletedAt;
    }

    public void setDeletedAt(Date deletedAt) {
        this.deletedAt = deletedAt;
    }

    public Date getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Date createdAt) {
        this.createdAt = createdAt;
    }

}
