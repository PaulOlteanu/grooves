import { useEffect, useState } from "react";
import useWebSocket from "react-use-websocket";
import { API_TOKEN } from "../../constants";
import api from "../../api";
import _ from "lodash";
import {
  ArrowBendDownLeft,
  ArrowBendDownRight,
  Pause,
  SkipBack,
  SkipForward,
  VinylRecord,
} from "@phosphor-icons/react";

const albumArt =
  "https://is1-ssl.mzstatic.com/image/thumb/Music116/v4/de/cc/ad/deccadc1-4251-ead2-6cc6-53316177a55e/067003259163.png/600x600bb.jpg";

function PlaybackControls() {
  return (
    <div>
      <button
        className="w-1/5"
        onClick={() => void api.sendPlayerCommand({ type: "prev_element" })}
      >
        <ArrowBendDownLeft height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() => void api.sendPlayerCommand({ type: "prev_song" })}
      >
        <SkipBack height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() => void api.sendPlayerCommand({ type: "pause" })}
      >
        <Pause height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() => void api.sendPlayerCommand({ type: "next_song" })}
      >
        <SkipForward height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() => void api.sendPlayerCommand({ type: "next_element" })}
      >
        <ArrowBendDownRight height={32} width={32} className="mx-auto" />
      </button>
    </div>
  );
}

type PlaybackInfo = {
  album_name: string;
  artist_name: string;
  image_url: string;
  playback_status: string;
  song_name: string;
};

function isPlaybackInfo(val: unknown): val is PlaybackInfo {
  const v = val as PlaybackInfo;
  return (
    v.album_name !== undefined &&
    v.artist_name !== undefined &&
    v.image_url !== undefined &&
    v.playback_status !== undefined &&
    v.song_name !== undefined
  );
}

export default function Player() {
  const { sendMessage, lastJsonMessage } = useWebSocket(
    "ws://localhost:4000/player"
  );

  const [playerState, setPlaybackInfo] = useState<PlaybackInfo | null>(null);

  useEffect(() => {
    sendMessage(API_TOKEN);
  }, []);

  useEffect(() => {
    if (lastJsonMessage === null || _.isEmpty(lastJsonMessage)) {
      setPlaybackInfo(null);
      return;
    }

    console.log(lastJsonMessage);
    if (isPlaybackInfo(lastJsonMessage)) {
      console.log("Setting playback info");
      setPlaybackInfo(lastJsonMessage);
    }
  }, [lastJsonMessage, setPlaybackInfo]);

  if (playerState) {
    return (
      <div className="mx-auto">
        <div className="text-center">
          <h1 className="text-2xl font-bold">{playerState.album_name}</h1>
        </div>

        <div className="max-h-[75%] flex">
          {playerState.image_url ? (
            <img
              src={playerState.image_url || undefined}
              className="max-h-full max-w-full object-contain"
            />
          ) : (
            <VinylRecord size="100%" />
          )}
        </div>

        <div className="text-center mt-2">
          <p>{playerState.song_name}</p>
          <p>{playerState.artist_name}</p>
        </div>

        <div className="mt-3 w-full">
          <div className="mx-auto max-w-sm">
            <PlaybackControls />
          </div>
        </div>
      </div>
    );
  } else {
    return <div>Nothing is playing</div>;
  }
}
