package com.hattmo.liquid.messages.http;

import java.util.HashMap;
import java.util.Optional;

/**
 * @author Hattmo
 */
public class HTTPResponseMessageTemplateStore {

    private final HashMap<String, MessageTemplate> messageTemplates = new HashMap<>();

    public void saveTemplate(HTTPResponseMessage m, String name) {

        MessageTemplate newTemplate = new MessageTemplate(m.getResponseCode(), m.getReasonPhrase(),
                m.getBody().orElse(new byte[0]), m.parameters);
        messageTemplates.put(name, newTemplate);
    }

    public HTTPResponseMessage getMessageFromTemplate(String name) {
        Optional<MessageTemplate> templateOpt = Optional.ofNullable(messageTemplates.get(name));
        if (templateOpt.isPresent()) {
            MessageTemplate template = templateOpt.get();
            HTTPResponseMessage outMessage = new HTTPResponseMessage(template.responseCode, template.reasonPhrase);
            outMessage.body = new byte[template.body.length];
            System.arraycopy(template.body, 0, outMessage.body, 0, template.body.length);
            outMessage.parameters = new HashMap<>(template.parameters);
            return outMessage;
        } else {
            System.err.println("No template found");
            return new HTTPResponseMessage();
        }
    }

    private class MessageTemplate {

        private final String responseCode;
        private final String reasonPhrase;
        private final byte[] body;
        private final HashMap<String, String> parameters;

        private MessageTemplate(String inResponseCode, String inReasonPhrase, byte[] inBody,
                HashMap<String, String> inParameters) {
            this.responseCode = inResponseCode;
            this.reasonPhrase = inReasonPhrase;
            this.body = new byte[inBody.length];
            System.arraycopy(inBody, 0, this.body, 0, this.body.length);
            parameters = new HashMap<>(inParameters);

        }
    }
}
