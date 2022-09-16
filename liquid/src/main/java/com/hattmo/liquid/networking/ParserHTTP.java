package com.hattmo.liquid.networking;

import java.io.BufferedOutputStream;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.io.OutputStream;
import java.util.LinkedList;
import java.util.Locale;
import java.util.Optional;
import com.hattmo.liquid.messages.http.HTTPMessage;
import com.hattmo.liquid.messages.http.HTTPRequestMessage;
import com.hattmo.liquid.messages.http.HTTPResponseMessage;

/**
 * @author Hattmo
 */
public class ParserHTTP implements Parser {

    private BufferedReader br;
    private BufferedOutputStream bos;

    public void setInputStream(InputStream is) {
        this.br = new BufferedReader(new InputStreamReader(is));
    }

    public void setOutputStream(OutputStream os) {
        this.bos = new BufferedOutputStream(os);
    }

    public void sendMessage(Message m) throws IllegalArgumentException, IOException {

        if (m instanceof HTTPMessage) {
            HTTPMessage outMessage = (HTTPMessage) m;
            bos.write(outMessage.getBytes());
            bos.flush();
        } else {
            throw new IllegalArgumentException();
        }
    }

    public void recieveMessage(MessageDispatcher messageDispach) throws IOException {
        String startLine;
        startLine = br.readLine();
        if (startLine == null) {
            throw new IOException();
        }
        String[] startLineItems = startLine.split(" ", 3);
        if (startLineItems.length == 3) {
            HTTPMessage mess;
            if (startLineItems[0].equals("HTTP/1.1") || startLineItems[0].equals("HTTP/1.0")) {
                mess = new HTTPResponseMessage(startLineItems[1], startLineItems[2]);
            } else {
                mess = new HTTPRequestMessage(startLineItems[0], startLineItems[1]);
            }
            while (true) {
                String param;
                param = br.readLine();
                if (param == null || param.equals("")) {
                    break;
                }
                String[] split = param.split(":");
                if (split.length == 2) {
                    mess.setParameter(split[0].trim().toLowerCase(Locale.US), split[1].trim());
                }
            }

            Optional<String> contentLength = mess.getParameter("content-length");
            if (contentLength.isPresent()) {
                try {
                    int conLen = Integer.parseInt(contentLength.get());
                    byte[] bodyOut = new byte[conLen];
                    for (int i = 0; i < conLen; i++) {
                        bodyOut[i] = (byte) br.read();
                    }
                    if (bodyOut.length != 0) {
                        mess.setBody(bodyOut);
                    }
                } catch (NumberFormatException | IOException e) {
                    System.out.println(e);
                    return;
                }
            }
            Optional<String> transferEncoding = mess.getParameter("transfer-encoding");
            if (transferEncoding.isPresent() && transferEncoding.get().equals("chunked")) {
                LinkedList<Byte> tempBytes = new LinkedList<>();
                int value = 0;
                while ((value = Integer.parseInt(br.readLine(), 16)) != 0) {
                    if (value != 0) {
                        for (int i = 0; i < value; i++) {
                            tempBytes.add((byte) br.read());
                        }
                        br.readLine();
                    }
                }
                byte[] bodyOut = new byte[tempBytes.size()];
                for (int i = 0; i < tempBytes.size(); i++) {
                    bodyOut[i] = tempBytes.get(i);
                }
                mess.setBody(bodyOut);
            }
            messageDispach.dispatch(mess);
        }
    }
}
