import { useEffect, useState } from "react";
import useWebSocket from "react-use-websocket";
import _ from "lodash";
import {
  ArrowBendDownLeft,
  ArrowBendDownRight,
  Pause,
  SkipBack,
  SkipForward,
  VinylRecord,
} from "@phosphor-icons/react";
import { useAuth } from "../../contexts/auth";

function PlaybackControls() {
  const { apiClient } = useAuth();

  if (!apiClient) {
    return null;
  }

  return (
    <div>
      <button
        className="w-1/5"
        onClick={() =>
          void apiClient.sendPlayerCommand({ type: "prev_element" })
        }
      >
        <ArrowBendDownLeft height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() => void apiClient.sendPlayerCommand({ type: "prev_song" })}
      >
        <SkipBack height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() => void apiClient.sendPlayerCommand({ type: "pause" })}
      >
        <Pause height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() => void apiClient.sendPlayerCommand({ type: "next_song" })}
      >
        <SkipForward height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/5"
        onClick={() =>
          void apiClient.sendPlayerCommand({ type: "next_element" })
        }
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
    `${import.meta.env.VITE_WS_URL}/player`,
    {
      shouldReconnect() {
        return true;
      },
      retryOnError: true,
    }
  );
  const { token } = useAuth();

  if (!token) {
    return null;
  }

  const [playerState, setPlaybackInfo] = useState<PlaybackInfo | null>(null);

  useEffect(() => {
    sendMessage(token);
  }, []);

  useEffect(() => {
    if (lastJsonMessage === null || _.isEmpty(lastJsonMessage)) {
      setPlaybackInfo(null);
      return;
    }

    if (isPlaybackInfo(lastJsonMessage)) {
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
