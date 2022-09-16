import React from "react";
import { NavLink } from "react-router-dom";
import styled from "styled-components";

const Frame = ({ children }: React.PropsWithChildren<{}>) => {
  return (
    <React.Fragment>
      <Header>
        <nav>
          <NavLink to="/template">Template</NavLink>
          <NavLink to="/globals">Globals</NavLink>
          <NavLink to="/data">Data</NavLink>
        </nav>
      </Header>
      <Article>{children}</Article>
    </React.Fragment>
  );
};

const headerHeight = "40px";

const Header = styled.header`
  height: ${headerHeight};
  display: grid;
  align-items: center;
`;
const Article = styled.article`
  height: calc(100% - ${headerHeight});
`;
export default Frame;
