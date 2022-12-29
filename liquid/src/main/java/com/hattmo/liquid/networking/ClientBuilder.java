package com.hattmo.liquid.networking;

import java.io.IOException;
import java.net.Socket;
import java.util.stream.Stream;
import javax.net.SocketFactory;
import javax.net.ssl.SSLSocketFactory;

/**
 * @author Hattmo
 */
public class ClientBuilder {

    private Parser parser;
    private boolean isSecure;
    private SocketFactory sockFactory;

    public ClientBuilder() {
        parser = new ParserOBJ();
        isSecure = false;
    }

    public ClientBuilder setParser(Parser parser) {
        this.parser = parser;
        return this;
    }

    public ClientBuilder setSecure() {
        this.isSecure = true;
        sockFactory = SSLSocketFactory.getDefault();
        return this;
    }

    /**
     *
     * @param host
     * @param port
     * @param messhandlers
     * @return
     * @throws IOException
     */

    public ConnectionInstance connect(String host, int port, MessageHandler... messhandlers) throws IOException {
        ConnectionInstance out;
        if (isSecure) {
            out = new ConnectionInstance(true, sockFactory.createSocket(host, port), parser);
        } else {
            out = new ConnectionInstance(true, new Socket(host, port), parser);
        }
        Stream.of(messhandlers).forEach((MessageHandler mh) -> out.addMessageHandler(mh));
        new Thread(out).start();
        return out;
    }
}
