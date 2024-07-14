import { useEffect, useState } from "react";
import {
  ArrowBendDownLeft,
  ArrowBendDownRight,
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
        className="w-1/4"
        onClick={() =>
          void apiClient.sendPlayerCommand({ type: "prev_element" })
        }
      >
        <ArrowBendDownLeft height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/4"
        onClick={() => void apiClient.sendPlayerCommand({ type: "prev_song" })}
      >
        <SkipBack height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/4"
        onClick={() => void apiClient.sendPlayerCommand({ type: "next_song" })}
      >
        <SkipForward height={32} width={32} className="mx-auto" />
      </button>
      <button
        className="w-1/4"
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
  artists: string;
  image_url: string;
  song_name: string;
};

function isPlaybackInfo(val: unknown): val is PlaybackInfo {
  const v = val as PlaybackInfo;
  return (
    v.album_name !== undefined &&
    v.artists !== undefined &&
    v.image_url !== undefined &&
    v.song_name !== undefined
  );
}

export default function Player() {
  const { apiClient } = useAuth();

  if (!apiClient) {
    return null;
  }

  const [playerState, setPlaybackInfo] = useState<PlaybackInfo | null>(null);

  const setPlayerState = (data_str: any) => {
    if (!data_str) {
      setPlaybackInfo(null);
      return;
    }

    const data: unknown = JSON.parse(data_str as string);
    if (data === null || isPlaybackInfo(data)) {
      setPlaybackInfo(data);
    }
  };

  useEffect(() => {
    const run = async () => {
      const token = await apiClient.getSseToken();
      const eventSource = new EventSource(
        `${import.meta.env.VITE_API_URL}/player?token=${token}`,
      );

      eventSource.onmessage = (e) => setPlayerState(e.data);
    };

    void run();
  }, []);

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
              className="max-h-full max-w-full object-contain mh-auto"
            />
          ) : (
            <VinylRecord size="100%" />
          )}
        </div>

        <div className="text-center mt-2">
          <p>{playerState.song_name}</p>
          <p>{playerState.artists}</p>
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
