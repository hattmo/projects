import React, { useState } from "react";
import EditForm from "../form/EditForm";
import Modal from "../modal/Modal";
import Table from "../table/Table";
import FullDiv from "../utility/FullDiv";

const DataItemsPage = () => {
  const [editModalOpen, setEditModalOpen] = useState(false);
  const [rowEditing, setRowEditing] = useState<number | undefined>();
  return (
    <FullDiv>
      <input
        type="button"
        onClick={() => {
          setEditModalOpen(true);
          setRowEditing(undefined);
        }}
        value="ADD"
      />
      <Modal
        isOpen={editModalOpen}
        onClose={() => {
          setEditModalOpen(false);
        }}
      >
        <EditForm
          row={rowEditing}
          onSubmit={() => {
            setEditModalOpen(false);
          }}
        />
      </Modal>
      <Table
        onEditClicked={(row) => {
          setEditModalOpen(true);
          setRowEditing(row);
        }}
      ></Table>
    </FullDiv>
  );
};

export default DataItemsPage;
