import { ChildProcessWithoutNullStreams, spawn } from "child_process";

let status: "Stopped";
let proc: ChildProcessWithoutNullStreams;

export const initialize = () => {
  status = "Stopped";
};

export const start_process = async (command: string, args: string[]) => {
  if (status !== "Stopped") {
    throw new Error("Process already running");
  }
  proc = spawn(command, args);
  proc.on("close", (code) => {
    status = "Stopped";
  });
  proc.on("error", (err) => {
    if (proc.killed === false) {
      proc.kill();
      status = "Stopped";
    }
  });
  proc.on("exit", (code) => {
    status = "Stopped";
  });
};

export const connect = (cb: () => void) => {
  const proc = spawn("ssh", ["releases"]);
  console.log("HELLO!");
  proc.stdout.on("data", (chunk) => {
    console.log(chunk.toString);
  });
  proc.on("error", (e) => {
    console.log(e.message);
  });
  proc.on("close", (code) => {
    console.log("IM HERE");
    if (code) {
      console.log(code.toString());
    } else {
      console.log("no error code");
    }
    cb();
  });
};
