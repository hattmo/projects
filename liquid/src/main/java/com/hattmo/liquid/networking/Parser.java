package com.hattmo.liquid.networking;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;

/**
 * @author Hattmo
 */
public interface Parser {
    public void setInputStream(InputStream is) throws IOException;

    public void setOutputStream(OutputStream os) throws IOException;

    public void sendMessage(Message m) throws IllegalArgumentException, IOException;

    public void recieveMessage(MessageDispatcher messageDispach) throws IOException;
}