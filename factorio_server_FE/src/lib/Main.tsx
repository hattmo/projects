import React, { useEffect, useState } from "react";

const Main = ({}) => {
  const [waitingStatus, setWaitingStatus] = useState(true);
  const [serverStatus, setServerStatus] = useState<
    "Unknown" | "Started" | "Stopped"
  >("Unknown");
  useEffect(() => {
    if (waitingStatus) {
      fetch("/api/server_status")
        .then((response) => {
          setWaitingStatus(false);
          if (response.ok) {
            return response.json() as Promise<{
              status: "Started" | "Stopped";
            }>;
          } else {
            throw new Error("Failed to fetch status");
          }
        })
        .then((data) => setServerStatus(data.status))
        .catch(() => {
          setServerStatus("Unknown");
        });
    }
  }, [waitingStatus]);
  return (
    <div>
      <div>{serverStatus}</div>
      <input
        type="button"
        disabled={waitingStatus}
        value={serverStatus == "Started" ? "Stop Server" : "Start Server"}
        onClick={() => {
          setWaitingStatus(true);
        }}
      />
    </div>
  );
};

export default Main;
