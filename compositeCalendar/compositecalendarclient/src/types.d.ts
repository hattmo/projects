type AccessRole = "reader" | "owner" | "freeBusyReader" | "writer";

export interface ISettingCalendar {
    id: string;
    name: string;
    accessRole: AccessRole;
}

export interface ISettingInputItem {
    cal: ISettingCalendar,
    regex: string,
    exclude: boolean,
}


export interface ISetting {
    inputItems: ISettingInputItem[],
    outputCal?: ISettingCalendar,
    startDate: string,
    endDate: string,
}