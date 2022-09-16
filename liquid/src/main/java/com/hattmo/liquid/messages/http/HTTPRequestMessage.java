package com.hattmo.liquid.messages.http;

import java.util.Map;

/**
 * @author Hattmo
 */
public class HTTPRequestMessage extends HTTPMessage {

    private static final long serialVersionUID = -7320596245272284428L;
    private String requestMethod;
    private String requestURI;

    public HTTPRequestMessage(String requestMethod, String requestURI) {
        this.requestMethod = requestMethod;
        this.requestURI = requestURI;

    }

    public HTTPRequestMessage() {
        this("GET", "/");
    }

    public String getRequestMethod() {
        return requestMethod;
    }

    public String getRequestURI() {
        return requestURI;
    }

    public void setRequestMethod(String requestMethod) {
        this.requestMethod = requestMethod;
    }

    public void setRequestURI(String requestURI) {
        this.requestURI = requestURI;
    }

    @Override
    public byte[] getBytes() {
        String out = this.requestMethod + " " + this.requestURI + " HTTP/1.1\r\n";
        for (Map.Entry<String, String> entry : parameters.entrySet()) {
            out += entry.getKey() + ": " + entry.getValue() + "\r\n";
        }
        out += "\r\n";
        byte[] headBytes = out.getBytes();
        if (body == null) {
            return headBytes;
        } else {
            int outlen = headBytes.length + body.length;
            byte[] outBytes = new byte[outlen];
            for (int i = 0; i < outBytes.length; i++) {
                if (i < headBytes.length) {
                    outBytes[i] = headBytes[i];
                } else {
                    outBytes[i] = body[i - headBytes.length];
                }
            }
            return outBytes;
        }
    }

}
