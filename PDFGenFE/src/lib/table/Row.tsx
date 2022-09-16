import React from "react";
import editImg from "./edit.png";
import deleteImg from "./delete.png";
import { useDeleteData } from "../DataContext";
import { TableItem, TableItemButton } from "./TableStyles";
interface Props {
  headers: string[];
  dataItem: DataItem;
  rowNum: number;
  onEditClicked: (row: number) => void;
}

const Row = ({ headers, dataItem, rowNum, onEditClicked }: Props) => {
  const deleteData = useDeleteData();
  const col = headers.map((header) => dataItem[header] ?? "");
  const isEvenRow = rowNum % 2 === 0;
  return (
    <React.Fragment>
      <TableItemButton
        isEvenRow={isEvenRow}
        onClick={() => {
          onEditClicked(rowNum);
        }}
      >
        <img height="15px" src={editImg} alt="" />
      </TableItemButton>
      {col.map((text, index) => (
        <TableItem key={index} isEvenRow={isEvenRow}>
          {text}
        </TableItem>
      ))}
      <TableItemButton
        isEvenRow={isEvenRow}
        onClick={() => {
          deleteData(rowNum);
        }}
      >
        <img height="15px" src={deleteImg} alt="" />
      </TableItemButton>
    </React.Fragment>
  );
};

export default Row;
