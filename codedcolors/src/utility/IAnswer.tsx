export default interface IAnswer {
  color: string[][];
  master: Array<{
    order: string[];
    reverse: boolean[];
  }>;
}
