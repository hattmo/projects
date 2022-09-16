package com.hattmo.liquid.networking;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotEquals;
import org.junit.Test;

public class MessageTest {
    @Test
    public void messageBuildTest() {
        Message mess = new Message();
        Message.Type type = mess.type;
        assertEquals(type, Message.Type.data);
        assertNotEquals(type, Message.Type.connect);
    }
}