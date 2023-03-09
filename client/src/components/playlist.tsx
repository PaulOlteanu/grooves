import { X, Trash } from "phosphor-react";
import usePlaylist from "../hooks/usePlaylist";
import type {
  PlaylistElement,
  Song as SongT,
  Playlist as PlaylistT,
} from "../types";
import { removeElement, removeSong, updateElement } from "../util/playlists";

function Song({
  song,
  onDelete,
}: {
  song: SongT;
  onDelete: (event: React.MouseEvent<HTMLButtonElement>) => void;
}) {
  return (
    <div className="flex">
      <span className="inline-block w-full whitespace-nowrap overflow-hidden text-ellipsis">
        {song.name}
      </span>
      <button type="button" onClick={onDelete}>
        <Trash size={18} className="items-center" />
      </button>
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
        <span className="inline-block w-full whitespace-nowrap overflow-hidden text-ellipsis">
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

  return (
    <div className="h-full max-h-full w-full overflow-auto">
      <h1 className="text-center text-2xl font-bold py-1">{playlist.name}</h1>
      <div className="w-full p-2">{elements}</div>
    </div>
  );
}
