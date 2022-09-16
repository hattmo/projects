import { RequestFunction, User } from "./helpers/ObjectDef";

export default (request: RequestFunction) => {
    return {
        ListUsersInAccount: async (accountId: string, params?: {
            search_term?: string,
            enrollment_type?: string,
            sort?: "username" | "email" | "sis_id" | "last_login",
            order?: "asc" | "desc",
        }): Promise<User[]> => {
            return await request(
                "GET",
                `/api/v1/accounts/${accountId}/users`,
                params,
            ) as User[];
        },
        EditAUser: async (id: string, body?: {
            user?: {
                name?: string,
                short_name?: string,
                sortable_name?: string,
                time_zone?: string,
                email?: string,
                locale?: string,
                avatar?: {
                    token?: string,
                    url?: string,
                }
                title?: string,
                bio?: string,
            },
        }): Promise<User> => {
            return await request(
                "PUT",
                `/api/v1/users/${id}`,
                undefined,
                body,
            ) as User;
        },
    };
};
