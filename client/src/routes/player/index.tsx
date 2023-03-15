import { useEffect } from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";

export default function Player() {
  const { sendMessage, lastMessage, readyState } = useWebSocket(
    "ws://localhost:4000/player"
  );

  useEffect(() => {
    if (lastMessage !== null) {
      console.log(lastMessage);
    }
  }, [lastMessage]);

  return (
    <div className="min-w-full h-full max-h-full min-h-full">
      <p>Player</p>
      <button onClick={() => sendMessage("hello")}>asdf</button>
    </div>
  );
}
