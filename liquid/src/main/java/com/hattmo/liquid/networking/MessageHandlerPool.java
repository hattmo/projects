/*
 * To change this license header, choose License Headers in Project Properties.
 * To change this template file, choose Tools | Templates
 * and open the template in the editor.
 */
package com.hattmo.liquid.networking;

import java.util.LinkedList;

/**
 * @author Hattmo
 */

public class MessageHandlerPool {

    private final LinkedList<MessageHandler> handlers;

    MessageHandlerPool() {
        this.handlers = new LinkedList<>();
    }

    synchronized void handleMessages(ConnectionInstance conn, Message mess) {
        handlers.stream().forEach((MessageHandler m) -> m.handleMessage(conn, mess));
    }

    synchronized void add(MessageHandler mh) {
        this.handlers.add(mh);
    }

    void remove(MessageHandler mh) {
        this.handlers.remove(mh);
    }
}
