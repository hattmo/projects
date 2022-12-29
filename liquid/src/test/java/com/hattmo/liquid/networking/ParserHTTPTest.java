package com.hattmo.liquid.networking;

import java.io.ByteArrayInputStream;

import org.junit.Test;

public class ParserHTTPTest {

    private static final String testOne = "bad data";

    @Test
    public void recieveMessageTest() {
        ParserHTTP parser = new ParserHTTP();
        parser.setInputStream(new ByteArrayInputStream(testOne.getBytes()));
    }

}
