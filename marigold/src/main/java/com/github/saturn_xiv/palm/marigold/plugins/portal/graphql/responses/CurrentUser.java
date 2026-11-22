package com.github.saturn_xiv.palm.marigold.plugins.portal.graphql.responses;

import java.io.Serializable;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

import com.github.saturn_xiv.palm.marigold.plugins.portal.models.Session;
import com.github.saturn_xiv.palm.marigold.plugins.portal.utils.CasbinEnforcer;
import com.google.protobuf.InvalidProtocolBufferException;

public class CurrentUser implements Serializable {
    public CurrentUser(Session session, CasbinEnforcer enforcer) throws InvalidProtocolBufferException {
        this.lang = session.user().getLang();
        this.timezone = session.user().getTimezone();
        this.name = session.user().getName();
        this.avatar = Optional.ofNullable(session.user().getAvatar());
        this.isAdministrator = enforcer.has(session.user().getId(), CasbinEnforcer.administrator());
        this.isRoot = enforcer.has(session.user().getId(), CasbinEnforcer.root());

        this.roles = new ArrayList<>();
        for (var it : enforcer.getImplicitRolesForUser(session.user().getId())) {
            if (it.hasCode()) {
                this.roles.add(it.getCode());
            }
        }
        this.permissions = new ArrayList<>();
        for (var rule : enforcer.getImplicitPermissionsForUser(session.user().getId())) {
            if (rule.getAction().hasCode()) {
                if (rule.getObject().hasAll()) {
                    this.permissions.add(new Permission(
                            rule.getAction().getCode(),
                            new Resource(rule.getObject().getType(), Optional.empty())));
                } else if (rule.getObject().hasId()) {
                    this.permissions.add(new Permission(
                            rule.getAction().getCode(),
                            new Resource(rule.getObject().getType(), Optional.of((int) rule.getObject().getId()))));
                }
            }
        }
    }

    public CurrentUser() {

    }

    private String lang;
    private String timezone;
    private String name;
    private Optional<String> avatar;
    private boolean isRoot;
    private boolean isAdministrator;
    private List<String> roles;
    private List<Permission> permissions;

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

    public String getName() {
        return name;
    }

    public void setName(String name) {
        this.name = name;
    }

    public Optional<String> getAvatar() {
        return avatar;
    }

    public void setAvatar(Optional<String> avatar) {
        this.avatar = avatar;
    }

    public boolean isRoot() {
        return isRoot;
    }

    public void setRoot(boolean isRoot) {
        this.isRoot = isRoot;
    }

    public boolean isAdministrator() {
        return isAdministrator;
    }

    public void setAdministrator(boolean isAdministrator) {
        this.isAdministrator = isAdministrator;
    }

    public List<String> getRoles() {
        return roles;
    }

    public void setRoles(List<String> roles) {
        this.roles = roles;
    }

    public List<Permission> getPermissions() {
        return permissions;
    }

    public void setPermissions(List<Permission> permissions) {
        this.permissions = permissions;
    }

}
