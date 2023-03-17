import { useEffect } from "react";
import useWebSocket, { ReadyState } from "react-use-websocket";
import { API_TOKEN } from "../../constants";
import api from "../../api";

export default function Player() {
  const { sendMessage, lastMessage, readyState } = useWebSocket(
    "ws://localhost:4000/player"
  );

  useEffect(() => {
    if (lastMessage !== null) {
      console.log(lastMessage);
    }
  }, [lastMessage]);

  useEffect(() => {
    sendMessage(API_TOKEN);
  }, []);

  const play_playlist = {
    type: "play",
    playlist_id: 1,
  };

  function f() {
    void api.sendPlayerCommand(play_playlist);
  }

  return (
    <div className="min-w-full h-full max-h-full min-h-full">
      <p>Player</p>
      <button onClick={() => f()}>asdf</button>
    </div>
  );
}
