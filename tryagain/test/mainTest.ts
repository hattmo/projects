// tslint:disable: no-unused-expression
import { expect } from "chai";
import tryagain from "../src/index";
import sinon from "sinon";

describe("Tests", () => {
    it("Should run the function 5 times if the function always throws", async () => {
        const fakeFunction = sinon.fake.throws(new Error("Error Thrown"));
        try {
            await tryagain(5, () => {
                fakeFunction();
            });
            expect.fail("should not complete");
        } catch (error) {
            expect(fakeFunction.callCount).to.be.equal(5);
        }
    });
    it("should run the function once if the function returns", async () => {
        const fakeFunction = sinon.fake.returns(true);
        try {
            await tryagain(5, () => {
                fakeFunction();
            });
        } catch (error) {
            expect.fail("Should not throw and error");
            return;
        }
        expect(fakeFunction.calledOnce).to.true;
    });
    it("should run until stop is called when the first argument is 0", async () => {
        const fakeFunction = sinon.fake.throws(new Error("Error Thrown"));
        try {
            await tryagain(0, () => {
                fakeFunction();
            }, (_error, attempt, exit) => {
                if (attempt === 7) {
                    exit();
                }
            });
        } catch (error) {
            expect(fakeFunction.callCount).to.equal(7);
            return;
        }
        expect.fail();
    });
    it("should work with an async function rejecting", async () => {
        const fakeFunction = sinon.fake.rejects(new Error("Error Thrown"));
        try {
            await tryagain(4, async () => {
                await fakeFunction();
                return true;
            });
            expect.fail();
        } catch (error) {
            expect(fakeFunction.callCount).to.equal(4);
        }
    });
});
