import { RequestFunction, AccountNotification } from "./helpers/ObjectDef";

export default (request: RequestFunction) => {

    return {

        indexOfActiveGlobalNotificationForTheUser: async (
            accountId: string,
        ): Promise<AccountNotification[]> => {
            return await request(
                "GET",
                `/api/v1/accounts/${accountId}/account_notifications`,
            ) as AccountNotification[];
        },

        showGlobalNotification: async (
            accountId: string,
            id: string,
        ): Promise<AccountNotification> => {
            return await request(
                "GET",
                `/api/v1/accounts/${accountId}/account_notifications/${id}`,
            ) as AccountNotification;
        },

        closeNotificationForUser: async (
            accountId: string,
            id: string,
        ): Promise<AccountNotification> => {
            return await request(
                "DELETE",
                `/api/v1/accounts/${accountId}/account_notifications/${id}`,
            ) as AccountNotification;
        },

        createAGlobalNotification: async (
            accountId: string,
            body: {
                "account_notification": {
                    subject: string,
                    message: string,
                    start_at: string,
                    end_at?: string,
                    icon?: string,
                }
                "account_notification_roles"?: string[],
            },
        ): Promise<AccountNotification> => {
            return await request(
                "POST",
                `/api/v1/accounts/${accountId}/account_notifications`,
                undefined,
                body,
            ) as AccountNotification;
        },

        updateAGlobalNotification: async (
            accountId: string,
            id: string,
            body: {
                "account_notification"?: {
                    subject?: string,
                    message?: string,
                    start_at?: string,
                    end_at?: string,
                    icon?: string,
                }
                "account_notification_roles"?: string[],
            },
        ): Promise<AccountNotification> => {
            return await request(
                "PUT",
                `/api/v1/accounts/${accountId}/account_notifications/${id}`,
                undefined,
                body,
            ) as AccountNotification;
        },

    };
};
