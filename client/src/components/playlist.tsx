import {
  X,
  Trash,
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
import Card from "./card";
import { useEffect, useState } from "react";

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

      <div className="hidden md:flex align-center">
        <button type="button" className="align-center" onClick={onDelete}>
          <Trash size={18} className="items-center" />
        </button>
      </div>
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
  let songs = null;
  if (!collapsed) {
    songs = element.songs.map((s, i) => {
      const onDelete = () => {
        const updatedElement = removeSong(element, i);
        onUpdate(updatedElement);
      };

      return <Song song={s} key={s.name} onDelete={onDelete} />;
    });

    songs = <div className="divide-y">{songs}</div>;
  }

  return (
    <div className="rounded border px-2 mb-4">
      <div className="flex">
        <button
          type="button"
          className="justify-self-left"
          onClick={toggleCollapse}
        >
          {collapsed ? <CaretRight size={24} /> : <CaretDown size={24} />}
        </button>

        <div className="flex-grow text-center">
          <span className="text-xl font-bold whitespace-nowrap overflow-hidden text-ellipsis align-middle">
            {element.name}
          </span>
          <button onClick={onPlay} className="align-middle">
            <Play size={24} />
          </button>
        </div>

        <button className="justify-self-right" type="button" onClick={onDelete}>
          <X size={24} />
        </button>
      </div>

      {songs}
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
        <div className="text-center py-1">
          <span className="text-2xl font-bold">{playlist.name}</span>

          <button
            onClick={() => void apiClient.sendPlayerCommand(playPlaylist)}
            className="float-right pr-2 inline-block"
          >
            <Play size={32} />
          </button>
        </div>

        <div className="flex underline">
          <button onClick={expandAll} className="flex-grow">
            <ArrowsOutLineVertical
              size={18}
              className="items-center inline-block mr-2"
            />

            <span>Expand All</span>
          </button>

          <button onClick={collapseAll} className="flex-grow">
            <ArrowsInLineVertical
              size={18}
              className="items-center inline-block mr-2"
            />

            <span>Collapse All</span>
          </button>
        </div>
      </div>

      <div className="w-full p-2">{elements}</div>
    </div>
  );
}
