// tslint:disable: no-unused-expression
import { expect } from "chai";
import index from "../src/index";
import sinon from "sinon";
import { mockRequest, mockResponse } from "mock-req-res";

const testSchema = {
    definitions: {},
    $schema: "http://json-schema.org/draft-07/schema#",
    $id: "http://example.com/root.json",
    type: "object",
    title: "The Root Schema",
    required: [
        "flight",
        "time",
    ],
    properties: {
        flight: {
            $id: "#/properties/flight",
            type: "string",
            title: "The Flight Schema",
            default: "",
            minLength: 3,
            maxLength: 3,
            pattern: "^(A|B|C|F)[0-9][0-9]$",
        },
        time: {
            $id: "#/properties/time",
            type: "integer",
            title: "The Time Schema",
            default: 0,
        },
    },
};
describe("Tests", () => {
    it("Should call next if the body is correct", () => {
        const middleware = index(testSchema);
        const nextSpy = sinon.spy();
        const req = mockRequest({
            body: {
                flight: "B17",
                time: 1000,
            },
        });
        const res = mockResponse();
        middleware(req, res, nextSpy);
        expect(nextSpy.calledOnce).to.be.true;
        expect(nextSpy.lastCall.args.length).to.equal(0);
    });

    it("Should call next with an error if the body is not correct", () => {
        const middleware = index(testSchema);
        const nextSpy = sinon.spy();
        const req = mockRequest({
            body: {
                flight: "B17",
            },
        });
        const res = mockResponse();
        middleware(req, res, nextSpy);
        expect(nextSpy.calledOnce).to.be.true;
        expect(nextSpy.lastCall.args.length).to.equal(1);
    });

});
