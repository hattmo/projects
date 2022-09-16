import React from "react";
import DataContext from "./DataContext";
import DataItemsPage from "./pages/DataItemsPage";
import {
  BrowserRouter as Router,
  Redirect,
  Route,
  Switch,
} from "react-router-dom";
import Frame from "./frame/Frame";
import FileDrop from "./fileDrop/FileDrop";
import { createGlobalStyle } from "styled-components";
import GlobalItemsPage from "./pages/GlobalItemsPage";
import TemplatePage from "./pages/TemplatePage";

const GlobalStyle = createGlobalStyle`
html {
  height: 100%;
}
body {
  height: 100%;
  margin: 0;
}
`;
const App = () => {
  return (
    <Router>
      <GlobalStyle />
      <DataContext>
        <Frame>
          <Switch>
            <Route path="/data">
              <FileDrop>
                <DataItemsPage />
              </FileDrop>
            </Route>
            <Route path="/globals">
              <GlobalItemsPage />
            </Route>
            <Route path="/template">
              <TemplatePage />
            </Route>
            <Route>
              <Redirect to="/data" />
            </Route>
          </Switch>
        </Frame>
      </DataContext>
    </Router>
  );
};

export default App;
