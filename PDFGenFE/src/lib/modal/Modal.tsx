import React from "react";
import styled from "styled-components";

interface Props {
  onClose: () => void;
  isOpen: boolean;
}

const Modal = ({
  onClose,
  isOpen,
  children,
}: React.PropsWithChildren<Props>) => {
  return isOpen ? (
    <ModalStyle
      onClick={() => {
        onClose();
      }}
      className="modalBackground"
    >
      <div
        onClick={(e) => {
          e.stopPropagation();
        }}
      >
        {children}
      </div>
    </ModalStyle>
  ) : null;
};

const ModalStyle = styled.div`
  height: 100%;
  width: 100%;
  left: 0;
  top: 0;
  position: fixed;
  display: grid;
  place-content: center;
  background-color: rgba(0, 0, 0, 0.5);
`;
export default Modal;
