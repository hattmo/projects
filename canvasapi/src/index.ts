import APITokenScopes from "./lib/APITokenScopes";
import AccountDomainLookups from "./lib/AccountDomainLookups";
import AccountNotifications from "./lib/AccountNotifications";
import webRequest from "./lib/helpers/WebRequest";
import https from "https";
import Users from "./lib/Users";
import Courses from "./lib/Courses";

export default (host: string, key: string, httpsOptions?: https.RequestOptions) => {
  const request = webRequest(host, key, httpsOptions);
  return {
    APITokenScopes: APITokenScopes(request),
    AccountDomainLookups: AccountDomainLookups(request),
    AccountNotifications: AccountNotifications(request),
    Users: Users(request),
    Courses: Courses(request),
  };
};
