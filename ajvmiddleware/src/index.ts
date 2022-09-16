import { Request, Response, NextFunction } from "express";
import Ajv from "ajv";

export default (schema: object) => {
  const ajv = new Ajv();
  const validate = ajv.compile(schema);
  return (req: Request, _res: Response, next: NextFunction) => {
    const isValid = validate(req.body);
    if (isValid) {
      next();
    } else {
      next(validate.errors);
    }
  };
};
