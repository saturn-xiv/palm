package com.github.saturn_xiv.palm.marigold.plugins.portal.models.user;

import java.io.Serializable;
import java.util.Date;

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
@Table(name = "email_users")
public class ByEmail implements Serializable {
    private static final long serialVersionUID = 1L;

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
    @Size(min = 2, max = 31)
    private String name;
    @Column(nullable = false)
    @Size(min = 5, max = 63)
    private String email;
    @Column(nullable = false)
    @Size(min = 64, max = 255)
    private String password;
    @Column(nullable = false)
    @Size(min = 9, max = 127)
    private String avatar;
    private Date confirmedAt;
    private Date lockedAt;
    private Date deletedAt;
    private int version;
    @Column(nullable = false)
    private Date updatedAt;
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

    public String getSn() {
        return sn;
    }

    public void setSn(String sn) {
        this.sn = sn;
    }

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

    public String getAvatar() {
        return avatar;
    }

    public void setAvatar(String avatar) {
        this.avatar = avatar;
    }

    public Date getConfirmedAt() {
        return confirmedAt;
    }

    public void setConfirmedAt(Date confirmedAt) {
        this.confirmedAt = confirmedAt;
    }

    public Date getLockedAt() {
        return lockedAt;
    }

    public void setLockedAt(Date lockedAt) {
        this.lockedAt = lockedAt;
    }

    public Date getDeletedAt() {
        return deletedAt;
    }

    public void setDeletedAt(Date deletedAt) {
        this.deletedAt = deletedAt;
    }

    public int getVersion() {
        return version;
    }

    public void setVersion(int version) {
        this.version = version;
    }

    public Date getUpdatedAt() {
        return updatedAt;
    }

    public void setUpdatedAt(Date updatedAt) {
        this.updatedAt = updatedAt;
    }

    public Date getCreatedAt() {
        return createdAt;
    }

    public void setCreatedAt(Date createdAt) {
        this.createdAt = createdAt;
    }

}
