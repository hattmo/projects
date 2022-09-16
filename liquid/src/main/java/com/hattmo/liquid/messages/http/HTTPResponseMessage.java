package com.hattmo.liquid.messages.http;

import java.util.Map.Entry;

/**
 * @author Hattmo
 */
public class HTTPResponseMessage extends HTTPMessage {

    private static final long serialVersionUID = -3724668275038525543L;
    private String responseCode;
    private String reasonPhrase;

    public HTTPResponseMessage(String responseCode, String reasonPhrase) {
        this.responseCode = responseCode;
        this.reasonPhrase = reasonPhrase;

    }

    public HTTPResponseMessage() {
        this("200", "OK");
    }

    public String getResponseCode() {
        return responseCode;
    }

    public String getReasonPhrase() {
        return reasonPhrase;
    }

    public void setResponseCode(String responseCode) {
        this.responseCode = responseCode;
    }

    public void setReasonPhrase(String reasonPhrase) {
        this.reasonPhrase = reasonPhrase;
    }

    @Override
    public byte[] getBytes() {
        String headerString = "HTTP/1.1 " + this.responseCode + " " + reasonPhrase + "\r\n";
        for (Entry<String, String> entry : parameters.entrySet()) {
            headerString += entry.getKey() + ": " + entry.getValue() + "\r\n";
        }
        headerString += "\r\n";
        byte[] headBytes = headerString.getBytes();
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
