import { RequestFunction, DomainLookup } from "./helpers/ObjectDef";

export default (request: RequestFunction) => {

    return {
        searchAccountDomains: async (
            params?: {
                name?: string,
                domain?: string,
                latitude?: number,
                longitude?: number,
            },
        ): Promise<DomainLookup> => {
            return await request(
                "GET",
                `/api/v1/accounts/search`,
                params,
            ) as DomainLookup;
        },
    };
};
