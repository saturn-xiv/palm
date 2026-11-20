package com.github.saturn_xiv.palm.marigold.wechat.responses;

import java.io.Serializable;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import com.fasterxml.jackson.annotation.JsonProperty;

public class ErrorResponse implements Serializable {
    public void check() {
        if (this.errorCode != null && this.errorCode != 0) {
            logger.error("{} {}", errorCode, errorMessage);
            throw new IllegalStateException(errorMessage);
        }
    }

    private @JsonProperty("errcode") Integer errorCode;
    private @JsonProperty("errmsg") String errorMessage;
    private final static Logger logger = LoggerFactory.getLogger(ErrorResponse.class);

    public Integer getErrorCode() {
        return errorCode;
    }

    public void setErrorCode(Integer errorCode) {
        this.errorCode = errorCode;
    }

    public String getErrorMessage() {
        return errorMessage;
    }

    public void setErrorMessage(String errorMessage) {
        this.errorMessage = errorMessage;
    }

}
