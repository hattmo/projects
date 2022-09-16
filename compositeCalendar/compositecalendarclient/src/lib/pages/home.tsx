import React from 'react'
import { Link } from 'react-router-dom'

const component = () => {
    return (
        <div>
            <h1>Home</h1>
            <br />
            <Link to="/pages/console">Console</Link>
            <br />
            <a href="/login">Login</a>
            <br />
        </div >
    )
}
component.displayName = "Home";
export default component;