package com.hattmo.liquid.messages.http;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.HashMap;
import java.util.Optional;
import java.util.zip.GZIPOutputStream;
import com.hattmo.liquid.networking.Message;

/**
 * @author Hattmo
 */
public abstract class HTTPMessage extends Message {

    private static final long serialVersionUID = -5676577157966569065L;

    protected HTTPMessage() {

    }

    protected byte[] body = new byte[0];
    protected HashMap<String, String> parameters = new HashMap<>();

    public void setParameter(String index, String value) {
        parameters.put(index, value);
    }

    public void setBody(byte[] body) {
        this.body = new byte[body.length];
        System.arraycopy(body, 0, this.body, 0, body.length);
        parameters.put("Content-Length", String.valueOf(this.body.length));
    }

    public void setBodyCompressed(byte[] body) throws IOException {

        ByteArrayOutputStream outByteStream = new ByteArrayOutputStream();
        GZIPOutputStream compressStream = new GZIPOutputStream(outByteStream);
        ByteArrayInputStream inByteStream = new ByteArrayInputStream(body);
        int inByte = inByteStream.read();
        while (inByte != -1) {
            compressStream.write(inByte);
            inByte = inByteStream.read();
        }
        compressStream.flush();
        compressStream.close();
        this.body = outByteStream.toByteArray();
        parameters.put("Content-Length", String.valueOf(this.body.length));
        parameters.put("Content-Encoding", "gzip");

    }

    public Optional<String> getParameter(String index) {
        return Optional.ofNullable(parameters.get(index));
    }

    public Optional<byte[]> getBody() {
        if (body.length != 0) {
            return Optional.of(body);
        } else {
            return Optional.empty();
        }
    }

    public abstract byte[] getBytes();
}
