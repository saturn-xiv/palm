package com.github.saturn_xiv.palm.camellia.controllers;

import org.springframework.http.MediaType;
import org.springframework.stereotype.Controller;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestPart;
import org.springframework.web.multipart.MultipartFile;

import com.github.saturn_xiv.palm.camellia.requests.FileUploadRequest;
import com.github.saturn_xiv.palm.camellia.responses.FileUploadResponse;

import jakarta.validation.Valid;

@Controller("palm.camellia.file-upload-controller")
public class FileUploadController {
    @PostMapping(path = "/upload", consumes = { MediaType.MULTIPART_FORM_DATA_VALUE })
    public FileUploadResponse saveEmployee(@Valid @RequestPart FileUploadRequest form,
            @RequestPart MultipartFile file) {
        // TODO
        FileUploadResponse reply = new FileUploadResponse("", "");
        return reply;
    }
}
