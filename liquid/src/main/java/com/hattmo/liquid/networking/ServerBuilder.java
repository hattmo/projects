package com.hattmo.liquid.networking;

import java.io.FileInputStream;
import java.io.IOException;
import java.net.InetAddress;
import java.net.ServerSocket;
import java.security.GeneralSecurityException;
import java.security.KeyStore;
import javax.net.ServerSocketFactory;
import javax.net.ssl.KeyManagerFactory;
import javax.net.ssl.SSLContext;

/**
 * @author Hattmo
 */
public class ServerBuilder {

    private Parser parser;
    private boolean isSecure;
    private boolean keyset;
    private ServerSocketFactory servSockFactory;

    public ServerBuilder() {
        parser = new ParserOBJ();
        isSecure = false;
    }

    public ServerBuilder setParser(Parser parser) {
        this.parser = parser;
        return this;
    }

    public ServerBuilder setKeystore(String keyStorePath, String storepass, String keypass)
            throws GeneralSecurityException, IOException {

        SSLContext context = SSLContext.getInstance("TLS");
        KeyManagerFactory keyFactory = KeyManagerFactory.getInstance("SunX509");
        KeyStore keystore = KeyStore.getInstance("JKS");
        keystore.load(new FileInputStream(keyStorePath), storepass.toCharArray());
        keyFactory.init(keystore, keypass.toCharArray());
        context.init(keyFactory.getKeyManagers(), null, null);
        servSockFactory = context.getServerSocketFactory();
        this.keyset = true;
        return this;
    }

    public ServerBuilder setSecure() {
        this.isSecure = true;
        return this;
    }

    public ServerConnectionListener listen(String bindAddress, int port, ConnectionHandler ch) throws IOException {
        InetAddress addr = InetAddress.getByName(bindAddress);
        ServerConnectionListener serverConnectionListener;
        if (isSecure && keyset) {
            serverConnectionListener = new ServerConnectionListener(servSockFactory.createServerSocket(port, 0, addr),
                    ch, parser);
        } else if (isSecure && !keyset) {
            throw new IllegalStateException();
        } else {
            serverConnectionListener = new ServerConnectionListener(new ServerSocket(port, 0, addr), ch, parser);
        }

        new Thread(serverConnectionListener).start();
        return serverConnectionListener;
    }

    public ServerConnectionListener listen(int port, ConnectionHandler ch) throws IOException {
        return listen("0.0.0.0", port, ch);
    }
}
