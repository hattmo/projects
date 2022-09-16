package com.hattmo.liquid.networking;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.Socket;

/**
 * @author Hattmo
 */
public class ConnectionInstance implements Runnable {

    private final Socket socket;
    private final InputStream is;
    private final OutputStream os;
    private final Parser parser;
    private final MessageHandlerPool handlerPool = new MessageHandlerPool();
    private boolean closed = false;

    public ConnectionInstance(boolean isClient, Socket s, Parser parser) throws IOException {
        this.closed = false;
        this.socket = s;
        if (isClient) {
            this.is = socket.getInputStream();
            this.os = socket.getOutputStream();
        } else {
            this.os = socket.getOutputStream();
            this.is = socket.getInputStream();
        }
        this.parser = parser;
        this.parser.setInputStream(is);
        this.parser.setOutputStream(os);
    }

    public void closeConnection() {
        if (!closed) {
            try {
                this.os.flush();
                this.is.close(); // close and flush the streams then close the socket
                this.os.close();
                this.socket.close();
            } catch (IOException ex) {

            } finally {
                Message temp = new Message();
                temp.type = Message.Type.disconnect;
                closed = true;
                handlerPool.handleMessages(this, temp);
            }
        }
    }

    public void sendMessage(Message m) throws IOException, IllegalArgumentException {
        try {
            parser.sendMessage(m);
        } catch (IOException e) {
            closeConnection();
            throw e;
        }
    };

    public boolean isClosed() {
        return closed;
    }

    public void addMessageHandler(MessageHandler mh) {
        this.handlerPool.add(mh);
    }

    public void removeMessageHandler(MessageHandler mh) {
        this.handlerPool.remove(mh);
    }

    @Override
    public void run() {
        Message temp = new Message();
        temp.type = Message.Type.connect; // indicate that a connection has been made
        handlerPool.handleMessages(this, temp);
        while (!closed) {
            try {
                parser.recieveMessage((m) -> {
                    handlerPool.handleMessages(this, m);
                });
            } catch (IOException e) {
                this.closeConnection();
            }
        }
    }

}
