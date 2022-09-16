import { RequestFunction, Scope } from "./helpers/ObjectDef";

export default (request: RequestFunction) => {
    return {
        listScopes: async (accountId: string, params?: { group_by?: string }): Promise<Scope> => {
            return await request(
                "GET",
                `/api/v1/accounts/${accountId}/scopes`,
                params,
            ) as Scope;
        },
    };
};
