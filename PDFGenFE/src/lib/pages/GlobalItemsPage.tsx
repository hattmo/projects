import React, { useState } from "react";
import { useGetGlobalData, useSetGlobalData } from "../DataContext";
import GlobalStateForm from "../form/GlobalStateForm";
import Modal from "../modal/Modal";
import evalTemplate from "../utility/evalTemplate";
import FullDiv from "../utility/FullDiv";

const GlobalItemsPage = () => {
  const globalData = useGetGlobalData();
  const setGlobalData = useSetGlobalData();
  const [modalOpen, setModalOpen] = useState(false);
  const interpolate = evalTemplate("blah${global.test}!", 0);
  return (
    <FullDiv>
      {Object.keys(globalData).map((item, index) => (
        <React.Fragment key={index}>
          <div>{item}</div>
          <textarea
            value={globalData[item]}
            onChange={(e) => {
              setGlobalData(item, e.target.value);
            }}
            cols={50}
            rows={5}
          />
        </React.Fragment>
      ))}
      <input
        type="button"
        value="ADD"
        onClick={() => {
          setModalOpen(true);
        }}
      />
      <Modal
        isOpen={modalOpen}
        onClose={() => {
          setModalOpen(false);
        }}
      >
        <GlobalStateForm
          onSubmit={() => {
            setModalOpen(false);
          }}
        />
      </Modal>
      {interpolate}
    </FullDiv>
  );
};

export default GlobalItemsPage;
