export default async function <T>(times: number,
                                  body: (attempt?: number) => Promise<T> | T,
                                  errorHandler?: (error: any, attempt: number, exit: () => void) => any): Promise<T> {
  let count = 1;
  while (count < times || times === 0) {
    try {
      return await body(count);
    } catch (iterationError) {
      let exitCalled = false;
      if (errorHandler) {
        await errorHandler(iterationError, count,
          () => {
            exitCalled = true;
          });
      }
      if (exitCalled) {
        throw iterationError;
      }
    }
    count++;
  }
  return await body(count);
}
