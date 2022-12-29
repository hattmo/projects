import React, { useState, useEffect } from "react";
import { BrowserRouter as Router, Route, Switch, Redirect } from "react-router-dom";
import Login from "./pages/login";
import Home from "./pages/home";
import Privacy from "./pages/privacy";
import Terms from "./pages/terms";
import Console from "./pages/console";

const app = () => {
    const [isLoggedIn, setIsLoggedIn] = useState(true);
    useEffect(() => {
        fetch("/api/session")
            .then(res => res.json())
            .then(body => {
                if (!body.loggedin) {
                    setIsLoggedIn(false);
                }
            }).catch(() => {
                setIsLoggedIn(false);
            })
    }, []);
    const logout = (_message: string) => {
        fetch("/api/session", { method: "DELETE", }).then(res => {
            if (res.ok) {
                setIsLoggedIn(false);
                console.log("logged out");
            } else {
                console.log("bad response from server not logged out");
            }
        })
    }
    return (
        <Router>
            <Switch>
                <Route path="/pages/login">
                    <Login />
                </Route>
                <Route path="/pages/privacy">
                    <Privacy />
                </Route>
                <Route path="/pages/terms">
                    <Terms />
                </Route>
                <Route path="/pages/console">
                    {
                        isLoggedIn ?
                            <Console logout={logout} /> :
                            <Redirect to="/" />
                    }
                </Route>
                <Route>
                    <Home />
                    <div>{isLoggedIn ? "logged in" : "not logged in"}</div>
                </Route>
            </Switch>
        </Router>
    );
};
app.displayName = "App";
export default app;