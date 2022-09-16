import crypto from "crypto-promise";
import IAnswer from "./IAnswer";

export function checkColors(colorSet: string[], subset: string, answers: IAnswer): number {
  const tempColors = [...colorSet];
  let answer: string[] = [];
  switch (subset) {
    case "A":
      answer = answers.color[0];
      break;
    case "B":
      answer = answers.color[1];
      break;
    case "C":
      answer = answers.color[2];
      break;
    case "D":
      answer = answers.color[3];
      break;
    default:
      return 0;
  }

  answer.forEach((val) => {
    const i = tempColors.indexOf(val);
    if (i >= 0) {
      tempColors.splice(i, 1);
    }
  });
  return 4 - tempColors.length;
}

export function checkPosition(colorSet: string[], subset: string, answers: IAnswer): number {
  let answer: string[] = [];
  switch (subset) {
    case "A":
      answer = answers.color[0];
      break;
    case "B":
      answer = answers.color[1];
      break;
    case "C":
      answer = answers.color[2];
      break;
    case "D":
      answer = answers.color[3];
      break;
    default:
      return 0;
  }
  let count = 0;
  colorSet.forEach((val, index) => {
    if (val === answer[index]) {
      count++;
    }
  });
  return count;
}

export function checkMaster(subsetOrder: string[], reversed: boolean[], answers: IAnswer) {
  const solutions = answers.master;
  let solved = false;
  solutions.forEach((solution) => {
    if (subsetOrder.every((val, index) => {
      return val === solution.order[index];
    }) &&
      reversed.every((val, index) => {
        return val === solution.reverse[index];
      })) {
      solved = true;
    }
  });
  return solved;
}

export const checkPassword = async (password: string): Promise<boolean> =>
  (await crypto.hash("md5")(password)).toString("hex") === "264eea33588766707d79bd1f9fe04439";

export const getAnswers = async (password: string): Promise<IAnswer> =>
  // tslint:disable-next-line: max-line-length
  JSON.parse(await crypto.decipher("aes256", password)("df00ef5a0cf8cc5c9494920f080d02b2254bc5a2597c819ec8ea37241917851b6c7991cb787c5350e97d8595b67a882332f6c53e7593ef7b2014187d3e5212f210415abebfea66c6a1b8e116cf522046b913f931d8ac4314c102cc9aa4c8e200f7a9fa0f60c52ca216e424e7c5996625c801a89309bdc48d26afa3ca53164c5f877617eb4aaa184924cb3308d70f1a9a5dd963791dd373e16f4b80f54d36cf233d0d7e9391ef9dc6560766a1ad92453acb716f841f21fc05576af426c93d700d0a43d131a6093373f14d90205b5256535ee0fbae0c2a3562607d017d1c7f58d339bc26ff5a530df54fd19a6cb60ac9abc7b0e09fe772bba56e395fec09fdc302c41b9ee59423b87f1be058d7e552c9ed0a0dd6d4a39015c79ef72e56ca8c8df5713ddda77624ab720ba90afb1935d39e63e830a4b07edb4f4ad23807339d7eff71f450ed73e1f86504c0d00c0fab5ff02764c65909f250b66375626f059fee3bb4fcc8478855ba02d28306a8d3c473ab942d5f6b1d6d523778fe9587256851eacf5ada3a31d526074cbbf568f500e2e5e1604f81a5e915da2be649ac0fa12ebe", "hex")) as IAnswer;
