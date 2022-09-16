package com.hattmo.liquid.networking;

import java.io.IOException;
import java.io.InputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.io.OutputStream;

/**
 * @author Hattmo
 */
public class ParserOBJ implements Parser {

    private ObjectInputStream ois;
    private ObjectOutputStream oos;

    public void setInputStream(InputStream is) throws IOException {
        this.ois = new ObjectInputStream(is);
    }

    public void setOutputStream(OutputStream os) throws IOException {
        this.oos = new ObjectOutputStream(os);
    }

    public void sendMessage(Message m) throws IOException {
        oos.writeObject(m);
        oos.flush();
    }

    @Override
    public void recieveMessage(MessageDispatcher messageDispach) throws IOException {
        try {
            Object inObj = this.ois.readObject();
            if (inObj instanceof Message) {
                Message inMess = (Message) inObj;
                messageDispach.dispatch(inMess);
            } else {
                throw new IOException();
            }
        } catch (ClassNotFoundException e) {
            throw new IOException(e);
        }
    }
}
