package com.hattmo.liquid.networking;

import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;

/**
 * @author Hattmo
 */
public class ServerConnectionListener implements Runnable {

    private final ServerSocket ss;
    private final ConnectionHandler ch;
    private final Parser parser;

    ServerConnectionListener(ServerSocket ss, ConnectionHandler ch, Parser parser) {
        this.ss = ss;
        this.ch = ch;
        this.parser = parser;
    }

    /**
     *
     * @throws IOException
     */
    public void closeConnection() throws IOException {
        ss.close();
    }

    @Override
    public void run() {
        Socket newSocket;
        ConnectionInstance instance;
        while (!ss.isClosed()) {
            try {
                newSocket = ss.accept();
                instance = new ConnectionInstance(false, newSocket, parser);
                ch.handleConnection(instance);
                new Thread(instance).start();
            } catch (IOException ex) {
            }
        }
    }
}
