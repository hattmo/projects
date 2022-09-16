package com.hattmo.liquid.networking;

import org.junit.Test;
import org.junit.Ignore;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.lang.reflect.Field;

public class ServerBuilderTest {

    @Test
    public void initializeServerBuilderTest()
            throws NoSuchFieldException, SecurityException, IllegalArgumentException, IllegalAccessException {
        ServerBuilder server = new ServerBuilder();

        Field parserField = server.getClass().getDeclaredField("parser");
        parserField.setAccessible(true);
        assertTrue(parserField.get(server) instanceof ParserOBJ);

        Field isSecureField = server.getClass().getDeclaredField("isSecure");
        isSecureField.setAccessible(true);
        assertEquals(isSecureField.get(server), false);
    }

    @Test
    public void setParserTest()
            throws NoSuchFieldException, SecurityException, IllegalArgumentException, IllegalAccessException {
        Parser mockParser = new MockParser();
        ServerBuilder builder = new ServerBuilder();
        Field parserField = builder.getClass().getDeclaredField("parser");
        parserField.setAccessible(true);

        builder.setParser(mockParser);
        assertEquals(mockParser, parserField.get(builder));
    }

    @Test
    @Ignore
    public void setKeystoreTest() {
        assert (true);
    }

    private class MockParser implements Parser {

        @Override
        public void setInputStream(InputStream is) throws IOException {

        }

        @Override
        public void setOutputStream(OutputStream os) throws IOException {

        }

        @Override
        public void sendMessage(Message m) throws IllegalArgumentException, IOException {

        }

        @Override
        public void recieveMessage(MessageDispatcher messageDispach) throws IOException {

        }

    }
}
