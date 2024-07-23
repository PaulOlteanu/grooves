import {
  X,
  Play,
  CaretRight,
  CaretDown,
  ArrowsInLineVertical,
  ArrowsOutLineVertical,
} from "@phosphor-icons/react";
import { useAuth } from "../contexts/auth";
import usePlaylist from "../hooks/usePlaylist";
import type {
  PlaylistElement,
  Song as SongT,
  Playlist as PlaylistT,
} from "../types";
import { removeElement, removeSong, updateElement } from "../util/playlists";
import { useEffect, useState } from "react";

function Song({
  song,
  onDelete,
}: {
  song: SongT;
  onDelete: (event: React.MouseEvent<HTMLButtonElement>) => void;
}) {
  // TODO: Some kind of padding on these
  return (
    <div className="group hover:bg-neutral-600/20 rounded flex">
      <li className="flex-grow truncate py-2">{song.name}</li>

      <button
        type="button"
        className="hidden group-hover:inline-block align-center float-right"
        onClick={onDelete}
      >
        <X size={18} className="items-center" />
      </button>
    </div>
  );
}

function Element({
  element,
  collapsed,
  toggleCollapse,
  onDelete,
  onUpdate,
  onPlay,
}: {
  element: PlaylistElement;
  onDelete: () => void;
  collapsed: boolean;
  toggleCollapse: () => void;
  onUpdate: (updatedElement: PlaylistElement) => void;
  onPlay: () => void;
}) {
  const songs = element.songs.map((s, i) => {
    const onDelete = () => {
      const updatedElement = removeSong(element, i);
      onUpdate(updatedElement);
    };

    return <Song song={s} key={s.name} onDelete={onDelete} />;
  });

  // TODO: a placeholder image
  // TODO: album cover change into play button on hover
  const padding = collapsed ? "pb-4" : "pb-2";
  return (
    <div className={"first:pt-0 last:pb-0 pt-4 " + padding}>
      <div className={"flex group"}>
        <img
          className="h-20 w-20"
          height={20}
          width={20}
          src={element.image_url || ""}
          alt=""
        />

        <div className="flex-grow px-2">
          <h2 className="text-2xl line-clamp-1 whitespace-pre-line">
            {element.name}
          </h2>
          <p className="line-clamp-1 whitespace-pre-line">{element.artists}</p>
        </div>

        <div className="flex-shrink-0 space-x-1">
          <button type="button" onClick={toggleCollapse}>
            {collapsed ? <CaretRight size={24} /> : <CaretDown size={24} />}
          </button>

          <button type="button" onClick={onPlay}>
            <Play size={24} />
          </button>

          <button type="button" onClick={onDelete}>
            <X size={24} />
          </button>
        </div>
      </div>

      {collapsed ? null : (
        <ol className="list-decimal list-inside pt-2">{songs}</ol>
      )}
    </div>
  );
}

type Collapsed = {
  [key: string]: boolean;
};

export default function Playlist({ playlist }: { playlist: PlaylistT }) {
  const { updatePlaylistMutation } = usePlaylist(playlist.id);
  const { apiClient } = useAuth();
  const [collapsed, setCollapsed] = useState<Collapsed>(() => {
    return playlist.elements.reduce(
      (acc, element) => ({ ...acc, [element.name]: false }),
      {},
    );
  });

  // TODO: Probably don't need useeffect
  useEffect(() => {
    setCollapsed({
      ...playlist.elements.reduce(
        (acc, element) => ({ ...acc, [element.name]: false }),
        {},
      ),
      ...collapsed,
    });
  }, [playlist]);

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

    const onPlay = () => {
      const playElement = {
        type: "play",
        playlist_id: playlist.id,
        element_index: i,
      };

      void apiClient.sendPlayerCommand(playElement);
    };

    const toggleCollapse = () => {
      const next = { ...collapsed, [e.name]: !collapsed[e.name] };

      setCollapsed(next);
    };

    return (
      <Element
        element={e}
        key={e.name}
        collapsed={collapsed[e.name]}
        toggleCollapse={toggleCollapse}
        onDelete={onDelete}
        onUpdate={onUpdate}
        onPlay={onPlay}
      />
    );
  });

  const playPlaylist = {
    type: "play",
    playlist_id: playlist.id,
  };

  const collapseAll = () => {
    setCollapsed(
      playlist.elements.reduce(
        (acc, element) => ({ ...acc, [element.name]: true }),
        {},
      ),
    );
  };

  const expandAll = () => {
    setCollapsed(
      playlist.elements.reduce(
        (acc, element) => ({ ...acc, [element.name]: false }),
        {},
      ),
    );
  };

  return (
    <div className="h-full max-h-full w-full overflow-auto">
      <div className="items-center">
        <div className="text-center py-1 px-4">
          <span className="text-2xl font-bold">{playlist.name}</span>

          <button
            onClick={() => void apiClient.sendPlayerCommand(playPlaylist)}
            className="float-right inline-block"
          >
            <Play size={32} />
          </button>
        </div>

        <div className="flex underline pb-2">
          <button onClick={expandAll} className="flex-grow">
            <ArrowsOutLineVertical
              size={20}
              className="items-center inline-block mr-2"
            />

            <span>Expand All</span>
          </button>

          <button onClick={collapseAll} className="flex-grow">
            <ArrowsInLineVertical
              size={20}
              className="items-center inline-block mr-2"
            />

            <span>Collapse All</span>
          </button>
        </div>
      </div>

      <hr className="mx-4" />

      <div className="w-full p-4 divide-y">
        {elements}
      </div>
    </div>
  );
}
