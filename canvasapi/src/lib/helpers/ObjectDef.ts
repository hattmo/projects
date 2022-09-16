export type RequestFunction = (
    method: string,
    path: string,
    parameters?: object | undefined,
    body?: object | undefined,
    headers?: object | undefined,
) => Promise<object>;

export interface User {
    "id": string;
    "name": string;
    "sortable_name": string;
    "short_name": string;
    "sis_user_id": string;
    "sis_import_id"?: string;
    "integration_id"?: string;
    "login_id": string;
    "avatar_url"?: string;
    "enrollments"?: null;   // Enrolments Needs to be figured out
    "email": string;
    "locale"?: string;
    "last_login"?: string;
    "time_zone"?: string;
    "bio"?: string;
}

export interface Scope {
    "resource": string;
    "resource_name": string;
    "controller": string;
}

export interface AccountNotification {
    "subject": string;
    "message": string;
    "start_at": string;
    "end_at": string;
    "icon": string;
    "roles": string[];
    "role_ids": string[];
}

export interface Grade {
    "html_url": string;
    "current_grade": string;
    "final_grade": string;
    "current_score": string;
    "final_score": string;
    "unposted_current_grade": string;
    "unposted_final_grade": string;
    "unposted_current_score": string;
    "unposted_final_score": string;
}

export interface Enrollments {
    "id": string;
    "course_id": string;
    "sis_course_id": string;
    "course_integration_id": string;
    "course_section_id": string;
    "section_integration_id": string;
    "sis_account_id": string;
    "sis_section_id": string;
    "sis_user_id": string;
    "enrollment_state": string;
    "limit_privileges_to_course_section": boolean;
    "sis_import_id": string;
    "root_account_id": string;
    "type": string;
    "user_id": string;
    "associated_user_id": null;
    "role": string;
    "role_id": string;
    "created_at": string;
    "updated_at": string;
    "start_at": string;
    "end_at": string;
    "last_activity_at": string;
    "last_attended_at": string;
    "total_activity_time": string;
    "html_url": string;
    "grades": Grade;
    "user": User;
    "override_grade": string;
    "override_score": string;
    "unposted_current_grade": string;
    "unposted_final_grade": string;
    "unposted_current_score": string;
    "unposted_final_score": string;
    "has_grading_periods": boolean;
    "totals_for_all_grading_periods_option": boolean;
    "current_grading_period_title": string;
    "current_grading_period_id": string;
    "current_period_override_grade": string;
    "current_period_override_score": string;
    "current_period_unposted_current_score": string;
    "current_period_unposted_final_score": string;
    "current_period_unposted_current_grade": string;
    "current_period_unposted_final_grade": string;
}

export interface DomainLookup {
    "name": string;
    "domain": string;
    "distance": string | null;
    "authentication_provider": string | null;
}
