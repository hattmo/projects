import styled, { css } from "styled-components";

const ItemStyle = css`
  padding: 2px;
  text-align: center;
`;

const ButtonStyle = css`
  display: grid;
  place-content: center;
  cursor: pointer;
`;

export const TableItem = styled.div<{ isEvenRow: boolean }>`
  ${ItemStyle}
  background-color: ${({ isEvenRow }) => (isEvenRow ? "white" : "grey")};
  color: black;
`;

export const TableItemButton = styled(TableItem)`
  ${ButtonStyle}
`;

export const HeaderItem = styled.div`
  ${ItemStyle}
  background-color: rgb(51, 51, 51);
  color: white;
`;

export const HeaderItemButton = styled(HeaderItem)`
  ${ButtonStyle}
`;
