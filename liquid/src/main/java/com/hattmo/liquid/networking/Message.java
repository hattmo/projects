package com.hattmo.liquid.networking;

import java.io.Serializable;

/**
 * @author Hattmo
 */
public class Message implements Serializable {

    private static final long serialVersionUID = 1;

    protected Message() {
        this.type = Message.Type.data;
    }

    public static enum Type {
        disconnect, connect, data
    }

    public Message.Type type;

}
