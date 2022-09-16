package com.hattmo.liquid.networking;

/**
 * @author Hattmo
 */
public interface MessageHandler {
    public void handleMessage(ConnectionInstance conn, Message mess);
}
