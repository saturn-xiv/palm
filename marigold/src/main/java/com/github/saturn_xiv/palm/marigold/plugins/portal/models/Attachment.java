package com.github.saturn_xiv.palm.marigold.plugins.portal.models;

import java.io.Serializable;
import java.time.Instant;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.annotations.SQLDelete;
import org.hibernate.annotations.SQLRestriction;
import org.hibernate.type.SqlTypes;
import org.springframework.data.annotation.CreatedDate;
import org.springframework.data.annotation.LastModifiedDate;
import org.springframework.data.jpa.domain.support.AuditingEntityListener;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.user.Item;

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
@Table(name = "attachments")
@SQLDelete(sql = "UPDATE attachments SET deleted_at = CURRENT_TIMESTAMP WHERE id = ?")
@SQLRestriction("deleted_at IS NULL")
public class Attachment implements Serializable {
    private static final long serialVersionUID = 1L;

    public sealed interface Storage extends Serializable permits S3, SeaweedFs {

    }

    public final record S3(String endpoint, String bucket, String object) implements Storage {
        private static final long serialVersionUID = 10L;
    }

    public final record SeaweedFs(String id) implements Storage {
        private static final long serialVersionUID = 20L;
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
    @Size(min = 2, max = 63)
    private String title;
    @Column(nullable = false)
    @Size(min = 3, max = 127)
    private String contentType;
    @Column(nullable = false)
    private int size;
    @Column(name = "public", nullable = false)
    private boolean public_;
    @JdbcTypeCode(SqlTypes.JSON)
    @Column(columnDefinition = "jsonb", nullable = false)
    private Storage storage;
    private Instant uploadedAt;
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

    public String getTitle() {
        return title;
    }

    public void setTitle(String title) {
        this.title = title;
    }

    public String getContentType() {
        return contentType;
    }

    public void setContentType(String contentType) {
        this.contentType = contentType;
    }

    public int getSize() {
        return size;
    }

    public void setSize(int size) {
        this.size = size;
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

    public boolean isPublic_() {
        return public_;
    }

    public void setPublic_(boolean public_) {
        this.public_ = public_;
    }

    public Storage getStorage() {
        return storage;
    }

    public void setStorage(Storage storage) {
        this.storage = storage;
    }

    public int getVersion() {
        return version;
    }

    public void setVersion(int version) {
        this.version = version;
    }

    public Instant getUploadedAt() {
        return uploadedAt;
    }

    public void setUploadedAt(Instant uploadedAt) {
        this.uploadedAt = uploadedAt;
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
