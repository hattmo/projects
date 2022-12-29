import React, { useState } from "react";
import HeaderForm from "../form/HeaderForm";
import Modal from "../modal/Modal";
import editImg from "./edit.png";
import { HeaderItem, HeaderItemButton } from "./TableStyles";

interface Props {
  headers: string[];
}
const Header = ({ headers }: Props) => {
  const [headerModalOpen, setHeaderModalOpen] = useState(false);
  return (
    <React.Fragment>
      <HeaderItemButton
        onClick={() => {
          setHeaderModalOpen(true);
        }}
      >
        <img height="15px" src={editImg} alt="" />
      </HeaderItemButton>
      {headers.map((header, i) => {
        return <HeaderItem key={i}>{header}</HeaderItem>;
      })}
      <HeaderItem />
      <Modal
        isOpen={headerModalOpen}
        onClose={() => {
          setHeaderModalOpen(false);
        }}
      >
        <HeaderForm
          onSubmit={() => {
            setHeaderModalOpen(false);
          }}
        />
      </Modal>
    </React.Fragment>
  );
};

export default Header;
