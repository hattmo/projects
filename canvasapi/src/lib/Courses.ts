import { RequestFunction, User } from "./helpers/ObjectDef";

export default (request: RequestFunction) => {
    return {
        ListUsersInCourse: async (courseId: string, params?: {
            search_term?: string,
            sort?: "username" | "email" | "sis_id" | "last_login",
            enrollment_type?: Array<"teacher" | "student" | "student_view" | "ta" | "observer" | "designer">,
            enrollment_role_id?: string,
            include?: Array<"enrollments" | "locked" | "avatar_url" | "test_student" | "bio" | "custom_links" | "current_grading_period_scores" | "uuid">,
            user_id?: string,
            user_ids?: string[],
            enrollment_state: Array<"active" | "invited" | "rejected" | "completed" | "inactive">,
        }): Promise<User[]> => {
            return await request(
                "GET",
                `/api/v1/courses/${courseId}/users`,
                params,
            ) as User[];
        },

    };
};
