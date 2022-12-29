package com.hattmo.liquid.messages.http;

import java.util.HashMap;
import java.util.Optional;

/**
 * @author Hattmo
 */
public class HTTPRequestMessageTemplateStore {

    private final HashMap<String, MessageTemplate> messageTemplates = new HashMap<>();

    public void saveTemplate(HTTPRequestMessage m, String name) {

        MessageTemplate newTemplate = new MessageTemplate(m.getRequestMethod(), m.getRequestURI(),
                m.getBody().orElse(new byte[0]), m.parameters);
        messageTemplates.put(name, newTemplate);
    }

    public HTTPRequestMessage getMessageFromTemplate(String name) {
        Optional<MessageTemplate> templateOpt = Optional.ofNullable(messageTemplates.get(name));
        if (templateOpt.isPresent()) {
            MessageTemplate template = templateOpt.get();
            HTTPRequestMessage outMessage = new HTTPRequestMessage(template.requestMethod, template.requestURI);
            outMessage.body = new byte[template.body.length];
            System.arraycopy(template.body, 0, outMessage.body, 0, template.body.length);
            outMessage.parameters = new HashMap<>(template.parameters);
            return outMessage;
        } else {
            System.err.println("No template found");
            return new HTTPRequestMessage();
        }

    }

    private class MessageTemplate {

        private final String requestMethod;
        private final String requestURI;
        private final byte[] body;
        private final HashMap<String, String> parameters;

        private MessageTemplate(String inRequestMethod, String inRequestURI, byte[] inBody,
                HashMap<String, String> inParameters) {
            this.requestMethod = inRequestMethod;
            this.requestURI = inRequestURI;
            this.body = new byte[inBody.length];
            System.arraycopy(inBody, 0, this.body, 0, this.body.length);
            parameters = new HashMap<>(inParameters);

        }
    }
}
