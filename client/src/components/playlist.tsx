import { X, Trash, Play } from "@phosphor-icons/react";
import { useAuth } from "../contexts/auth";
import usePlaylist from "../hooks/usePlaylist";
import type {
  PlaylistElement,
  Song as SongT,
  Playlist as PlaylistT,
} from "../types";
import { removeElement, removeSong, updateElement } from "../util/playlists";
import Card from "./card";

function Song({
  song,
  onDelete,
}: {
  song: SongT;
  onDelete: (event: React.MouseEvent<HTMLButtonElement>) => void;
}) {
  return (
    <div className="flex">
      <div className="flex flex-grow min-w-0">
        <Card
          content={song.name}
          subContent={song.artists}
          imageUrl={song.image_url}
        />
      </div>
      <div className="flex align-center">
        <button type="button" className="align-center" onClick={onDelete}>
          <Trash size={18} className="items-center" />
        </button>
      </div>
    </div>
  );
}

function Element({
  element,
  onDelete,
  onUpdate,
}: {
  element: PlaylistElement;
  onDelete: () => void;
  onUpdate: (updatedElement: PlaylistElement) => void;
}) {
  const songs = element.songs.map((s, i) => {
    const onDelete = () => {
      const updatedElement = removeSong(element, i);
      onUpdate(updatedElement);
    };

    return <Song song={s} key={s.name} onDelete={onDelete} />;
  });

  return (
    <div className="rounded border px-2 mb-4">
      <div className="flex">
        <span className="text-center text-xl font-bold inline-block w-full whitespace-nowrap overflow-hidden text-ellipsis">
          {element.name}
        </span>
        <button type="button" onClick={onDelete}>
          <X size={18} className="items-center" />
        </button>
      </div>

      <div className="divide-y">{songs}</div>
    </div>
  );
}

export default function Playlist({ playlist }: { playlist: PlaylistT }) {
  const { updatePlaylistMutation } = usePlaylist(playlist.id);
  const { apiClient } = useAuth();

  if (!apiClient) {
    return null;
  }

  const elements = playlist.elements.map((e, i) => {
    const onDelete = () => {
      const updatedPlaylist = removeElement(playlist, i);
      updatePlaylistMutation.mutate(updatedPlaylist);
    };

    const onUpdate = (updatedElement: PlaylistElement) => {
      const updatedPlaylist = updateElement(playlist, i, updatedElement);
      updatePlaylistMutation.mutate(updatedPlaylist);
    };

    return (
      <Element
        element={e}
        key={e.name}
        onDelete={onDelete}
        onUpdate={onUpdate}
      />
    );
  });

  const play_playlist = {
    type: "play",
    playlist_id: playlist.id,
  };

  return (
    <div className="h-full max-h-full w-full overflow-auto">
      <div className="text-center items-center">
        <p className="text-center underline text-2xl font-bold py-1">
          {playlist.name}
          <button
            onClick={() => void apiClient.sendPlayerCommand(play_playlist)}
            className="float-right pr-2 inline-block"
          >
            <Play size={32} />
          </button>
        </p>
      </div>
      <div className="w-full p-2">{elements}</div>
    </div>
  );
}
