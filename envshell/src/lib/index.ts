import { spawn as pty } from "node-pty";
import { Command } from "commander";
import { homedir } from "os";
import { join as p, sep } from "path";
import { stat, mkdir, writeFile, access, readFile } from "fs/promises";
import readline from "readline";
import {
  randomFill,
  scrypt,
  createDecipheriv,
  createHash,
  createCipheriv,
} from "crypto";

const envshelldir = p(homedir(), ".envshell");
const envshellConfPath = p(homedir(), ".envshell/conf.json");

interface Configuration {
  [path: string]: PathVarsEnc | PathVarsPlain | undefined;
}

interface PathVarsEnc {
  encrypted: true;
  salt: string;
  iv: string;
  hash: string;
  vars: string;
}
interface PathVarsPlain {
  encrypted: false;
  vars: Var[];
}

interface Var {
  type: OptString;
  key: string;
  value: string;
}

interface Options {
  prepend?: string;
  append?: string;
}

type OptString = "Replace" | "Prepend" | "Append";

type Environs = Array<{ path: string; env: NodeJS.ProcessEnv }>;

export default async () => {
  const path = process.cwd();
  await setupConfFolder();
  const conf = await loadConf();

  const envshell = new Command("envshell");
  envshell
    .action(startShell(conf, path))
    .description(
      "Start a shell with configured variables loaded into the environment"
    );
  envshell
    .command("set <key> <value>")
    .description(
      "Add or modify a variable in the environment. use '-' for the value to read from stdin"
    )
    .action(setEnvVar(conf, path));

  envshell
    .command("clear <key>")
    .description("Clear a variable in the environment")
    .action(clearEnvVar(conf, path));

  envshell
    .command("list")
    .description("List variables in this environment")
    .action(listEnvVar(conf, path));
  envshell
    .command("encrypt")
    .description("Encrypt the variables in the environment")
    .action(encrypt(conf, path));
  envshell
    .command("decrypt")
    .description("Remove the encryption on the variables in the environment")
    .action(decrypt(conf, path));

  envshell.option("-p, --prepend", "Prepend to existing variable", false);
  envshell.option("-a, --append", "Append to existing variable", false);
  envshell.parse();
};

const setupConfFolder = async () => {
  try {
    const dirstat = await stat(envshelldir);
    if (dirstat.isDirectory()) {
    } else {
      console.log(`Cannot open directory ${envshelldir}`);
    }
  } catch {
    mkdir(envshelldir, { recursive: true });
    console.log(`Created directory ${envshelldir}`);
  }
  try {
    await access(envshellConfPath);
  } catch {
    writeFile(envshellConfPath, JSON.stringify({}), { encoding: "utf-8" });
    console.log(`Created file ${envshellConfPath}`);
  }
};

const getEnviron = async (
  conf: Configuration,
  path: string
): Promise<Environs> => {
  const pathParts = path.split(sep);
  return await recurse(conf, pathParts);
};

const recurse = async (
  conf: Configuration,
  pathParts: string[]
): Promise<Environs> => {
  const path = pathParts.join(sep);
  const pathVars = conf[path];
  let environ: NodeJS.ProcessEnv = {};
  if (pathVars === undefined) {
    environ = {};
  } else if (pathVars.encrypted === true) {
    console.log(`Decrypting Variables for ${path}`);
    const password = await getPassword();
    const decPathVars = await decryptVars(pathVars, password);
    environ = convertVarsToEnv(decPathVars);
  } else {
    environ = convertVarsToEnv(pathVars);
  }
  pathParts.pop();
  if (pathParts.length === 1) {
    return [{ env: environ, path: path }];
  } else {
    const out = (await recurse(conf, pathParts)).concat([
      { env: environ, path: path },
    ]);
    return out;
  }
};

const loadConf = async (): Promise<Configuration> => {
  try {
    const confData = await readFile(envshellConfPath);
    return JSON.parse(confData.toString("utf-8"));
  } catch {
    console.log(`Error reading from ${envshellConfPath}`);
    process.exit();
  }
};

const getShell = (): string => {
  if (process.platform === "win32") {
    return "powershell.exe";
  }
  return "/bin/bash";
};

const startShell = (conf: Configuration, path: string) => async () => {
  if (process.env.envshell === "true") {
    console.log("You are already in an envshell...");
    process.exit();
  }
  process.stdout.write("\n");
  process.stdout.write(
    "*********************************************************\n"
  );
  process.stdout.write(
    "* Entered Envshell, activating environment variables... *\n"
  );
  process.stdout.write(
    "*********************************************************\n"
  );
  process.stdout.write("\n");
  const env = await getEnviron(conf, path);
  displayEnvVars(env);
  const mergeEnv = env
    .map((i) => i.env)
    .reduce((prev, curr) => {
      return { ...prev, ...curr };
    }, {});
  process.stdin.setRawMode(true);
  const proc = pty(getShell(), [], {
    cols: process.stdout.columns,
    rows: process.stdout.rows,
    env: { ...process.env, ...mergeEnv, envshell: "true" },
  });
  process.stdout.on("resize", () => {
    proc.resize(process.stdout.columns, process.stdout.rows);
  });
  proc.onData((data) => {
    process.stdout.write(data);
  });
  process.stdin.on("data", (data) => {
    proc.write(data.toString("utf-8"));
  });
  proc.onExit(() => {
    process.stdout.write("\n");
    process.stdout.write(
      "***********************************************************\n"
    );
    process.stdout.write(
      "* Leaving Envshell, deactivating environment variables... *\n"
    );
    process.stdout.write(
      "***********************************************************\n"
    );
    process.stdout.write("\n");
    process.exit();
  });
};

const getValue = () =>
  new Promise<string>((res, rej) => {
    let value = "";
    process.stdin.on("data", (data) => {
      value += data.toString("utf-8");
    });
    process.stdin.on("end", () => {
      res(value);
    });
    process.stdin.on("error", (err) => {
      rej(err);
    });
  });

const setEnvVar =
  (conf: Configuration, path: string) =>
  async (key: string, value: string, { append, prepend }: Options) => {
    let opt: OptString = "Replace";
    if (append) {
      opt = "Append";
    } else if (prepend) {
      opt = "Prepend";
    }
    let trueValue;
    if (value === "-") {
      trueValue = await getValue();
    } else {
      trueValue = value;
    }
    let pathConf = conf[path];
    let password = "";
    let encrypted = false;
    if (pathConf === undefined) {
      pathConf = {
        encrypted: false,
        vars: [{ key, value: trueValue, type: opt }],
      };
    } else if (pathConf.encrypted === true) {
      encrypted = true;
      password = await getPassword();
      pathConf = await decryptVars(pathConf, password);
    }
    const index = pathConf.vars.findIndex((val) => val.key === key);
    if (index !== -1) {
      pathConf.vars[index] = { key, value: trueValue, type: opt };
    } else {
      pathConf.vars.push({ key, value: trueValue, type: opt });
    }

    if (encrypted) {
      pathConf = await encryptVars(pathConf, password);
    }

    conf[path] = pathConf;

    try {
      await writeFile(envshellConfPath, JSON.stringify(conf), {
        encoding: "utf-8",
      });
      console.log(`Saved ${key}:${trueValue}`);
    } catch {
      console.log(`Failed to save ${key}:${trueValue}`);
    } finally {
      process.exit();
    }
  };

const listEnvVar = (conf: Configuration, path: string) => async () => {
  const env = await getEnviron(conf, path);
  displayEnvVars(env);
  process.exit();
};

const displayEnvVars = (env: Environs) => {
  env.forEach((item) => {
    const env = item.env;
    if (Object.keys(env).length > 0) {
      console.log(item.path);
      console.log("-------------");
      Object.keys(env).forEach((key) => {
        console.log(`${key} : ${env[key]}`);
      });
      console.log();
    }
  });
};

const clearEnvVar =
  (conf: Configuration, path: string) => async (key: string) => {
    let pathConf = conf[path];
    let password = "";
    let encrypted = false;
    if (pathConf === undefined) {
      console.log(`The environment ${path} has no variables set`);
      process.exit();
    }
    if (pathConf.encrypted === true) {
      encrypted = true;
      password = await getPassword();
      pathConf = await decryptVars(pathConf, password);
    }

    pathConf.vars = pathConf.vars.filter((item) => {
      item.key !== key;
    });
    if (encrypted) {
      pathConf = await encryptVars(pathConf, password);
    }
    conf[path] = pathConf;
    try {
      await writeFile(envshellConfPath, JSON.stringify(conf), "utf-8");
      console.log(`Removed variable ${key} from the environment`);
    } catch {
      console.log(`Failed to remove variable ${key} from the environment`);
    } finally {
      process.exit();
    }
  };

const decryptVars = (
  encConf: PathVarsEnc,
  password: string
): Promise<PathVarsPlain> =>
  new Promise((res, rej) => {
    const decConf: PathVarsPlain = { encrypted: false, vars: [] };
    const salt = Buffer.from(encConf.salt, "base64");
    const iv = Buffer.from(encConf.iv, "base64");
    const pass = Buffer.from(password, "utf-8");
    const combo = Buffer.concat([salt, pass]);
    const hash = createHash("sha256");
    hash.update(combo);
    if (encConf.hash !== hash.digest("base64")) {
      console.error("Incorrect Password");
      process.exit(1);
    }
    scrypt(pass, salt, 32, (err, key) => {
      if (err) {
        rej(err);
      }
      const cipher = createDecipheriv("aes-256-gcm", key, iv);
      let decrypted = cipher.update(encConf.vars, "base64", "utf-8");
      decrypted += cipher.final("utf-8");
      decConf.vars = JSON.parse(decrypted);
      res(decConf);
    });
  });

const encryptVars = (
  decConf: PathVarsPlain,
  password: string
): Promise<PathVarsEnc> =>
  new Promise((res, rej) => {
    randomFill(Buffer.alloc(16), (iverr, iv) => {
      if (iverr) {
        rej(iverr);
      }
      randomFill(Buffer.alloc(16), (salterr, salt) => {
        if (salterr) {
          rej(salterr);
        }
        const pass = Buffer.from(password, "utf-8");
        const combo = Buffer.concat([salt, pass]);
        const hash = createHash("sha256");
        hash.update(combo);
        scrypt(pass, salt, 32, (keyerr, key) => {
          if (keyerr) {
            rej(keyerr);
          }
          const cipher = createCipheriv("aes-256-gcm", key, iv);
          let encrypted = cipher.update(
            JSON.stringify(decConf.vars),
            "utf-8",
            "base64"
          );
          encrypted = cipher.final("base64");
          const encConf: PathVarsEnc = {
            encrypted: true,
            iv: iv.toString("base64"),
            salt: salt.toString("base64"),
            hash: hash.digest("base64"),
            vars: encrypted,
          };
          res(encConf);
        });
      });
    });
  });

const convertVarsToEnv = ({ vars }: PathVarsPlain): NodeJS.ProcessEnv => {
  const out: NodeJS.ProcessEnv = {};
  vars.forEach(({ key, type, value }) => {
    switch (type) {
      case "Replace":
        out[key] = value;
        break;
      case "Append":
        out[key] = process.env[key] + value;
        break;
      case "Prepend":
        out[key] = value + process.env[key];
        break;
    }
  });
  return out;
};

const getPassword = (): Promise<string> =>
  new Promise((res) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout,
    });
    rl.question(`Please enter password:\n`, (answer) => {
      res(answer);
    });
  });

const encrypt = (conf: Configuration, path: string) => async () => {
  const pathConf = conf[path];
  if (pathConf === undefined) {
    console.log(`No variables defined in this path: ${path}`);
    process.exit(0);
  } else if (pathConf.encrypted == true) {
    console.log(
      `Variables are already encrypted. Decrypt the variables first with envshell decrypt.`
    );
    process.exit(0);
  }
  const password = await getPassword();
  const newVars = await encryptVars(pathConf, password);
  conf[path] = newVars;
  try {
    await writeFile(envshellConfPath, JSON.stringify(conf), "utf-8");
    console.log(`Encrypted variables successfully`);
  } catch {
    console.log(`Failed to encrypt variables.`);
  } finally {
    process.exit();
  }
};

const decrypt = (conf: Configuration, path: string) => async () => {
  const pathConf = conf[path];
  if (pathConf === undefined) {
    console.log(`No variables defined in this path: ${path}`);
    process.exit(0);
  } else if (pathConf.encrypted == false) {
    console.log(`Variables are already decrypted.`);
    process.exit(0);
  }
  const password = await getPassword();
  const newVars = await decryptVars(pathConf, password);
  conf[path] = newVars;
  try {
    await writeFile(envshellConfPath, JSON.stringify(conf), "utf-8");
    console.log(`Encrypted variables successfully`);
  } catch {
    console.log(`Failed to encrypt variables.`);
  } finally {
    process.exit();
  }
};
