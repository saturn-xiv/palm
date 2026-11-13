package com.github.saturn_xiv.palm.marigold.plugins.hydrangea.models;

import java.io.Serializable;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;

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
@Table(name = "hydrangea_job_logs")
public class JobLog implements Serializable {
    private static final long serialVersionUID = 1L;

    public JobLog() {
        this.args = new ArrayList<>();
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
    @Size(min = 7, max = 45)
    private String ip;
    @Column(nullable = false)
    @Size(min = 2, max = 31)
    private String name;
    @Column(nullable = false)
    @Size(min = 1, max = 15)
    private String version;
    @Size(max = 511)
    private String summary;
    @JdbcTypeCode(SqlTypes.JSON)
    @Column(columnDefinition = "jsonb", nullable = false)
    private List<String> args;
    @Column(nullable = false)
    private String script;
    private Date launchedAt;
    private String elapsed;
    private Integer status_code;
    private String stdout;
    private String stderr;
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

    public String getIp() {
        return ip;
    }

    public void setIp(String ip) {
        this.ip = ip;
    }

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public String getVersion() {
        return version;
    }

    public void setVersion(String version) {
        this.version = version;
    }

    public String getSummary() {
        return summary;
    }

    public void setSummary(String summary) {
        this.summary = summary;
    }

    public List<String> getArgs() {
        return args;
    }

    public void setArgs(List<String> args) {
        this.args = args;
    }

    public String getScript() {
        return script;
    }

    public void setScript(String script) {
        this.script = script;
    }

    public Date getLaunchedAt() {
        return launchedAt;
    }

    public void setLaunchedAt(Date launchedAt) {
        this.launchedAt = launchedAt;
    }

    public String getElapsed() {
        return elapsed;
    }

    public void setElapsed(String elapsed) {
        this.elapsed = elapsed;
    }

    public Integer getStatus_code() {
        return status_code;
    }

    public void setStatus_code(Integer status_code) {
        this.status_code = status_code;
    }

    public String getStdout() {
        return stdout;
    }

    public void setStdout(String stdout) {
        this.stdout = stdout;
    }

    public String getStderr() {
        return stderr;
    }

    public void setStderr(String stderr) {
        this.stderr = stderr;
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
