import React from "react";
import { useGetData, useGetHeaders } from "../DataContext";
import Header from "./Header";
import Row from "./Row";
import styled from "styled-components";
interface Props {
  onEditClicked: (row: number) => void;
}
const Table = ({ onEditClicked }: Props) => {
  const headers = useGetHeaders();
  const data = useGetData();

  return (
    <TableStyle numCols={headers.length}>
      <Header headers={headers} />
      {data.map((dataItem, index) => {
        return (
          <Row
            onEditClicked={onEditClicked}
            rowNum={index}
            dataItem={dataItem}
            headers={headers}
            key={index}
          />
        );
      })}
    </TableStyle>
  );
};

const TableStyle = styled.div<{ numCols: number }>`
  padding: 1px;
  gap: 1px;
  background-color: black;
  display: grid;
  grid-template-columns: 20px repeat(${(props) => props.numCols}, 1fr) 20px;
`;
export default Table;
