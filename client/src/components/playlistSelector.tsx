import _ from "lodash";
import { Plus, X } from "@phosphor-icons/react";
import { useState } from "react";
import { NavLink } from "react-router-dom";
import usePlaylists from "../hooks/usePlaylists";
import type { Playlist as PlaylistT } from "../types";
import { useAuth } from "../contexts/auth";
import usePlaylist from "../hooks/usePlaylist";

function Playlist({ playlist }: { playlist: PlaylistT }) {
  const { deletePlaylistMutation } = usePlaylist(playlist.id);

  // TODO: If active and delete is pressed, redirect to /playlists
  return (
    <div className="px-2">
      <NavLink to={`/playlists/${playlist.id}`} className="group">
        {({ isActive }) => {
          let bg = "";
          if (isActive) {
            bg = "bg-neutral-400/20 hover:bg-neutral-200/20";
          } else {
            bg = "hover:bg-neutral-600/20";
          }

          return (
            <div className={"flex rounded py-2 px-2 " + bg}>
              <div className="flex inline-block w-full items-center min-w-0">
                <span className="whitespace-nowrap overflow-hidden text-ellipsis">
                  <span>{playlist.name}</span>
                </span>
              </div>
              <button
                className="z-100"
                type="button"
                onClick={(e) => {
                  e.preventDefault();
                  deletePlaylistMutation.mutate(playlist);
                }}
              >
                <X size={18} />
              </button>
            </div>
          );
        }}
      </NavLink>
    </div>
  );
}

export default function PlaylistSelector({
  playlists,
}: {
  playlists: PlaylistT[];
}) {
  const [searchFilter, setSearchFilter] = useState("");
  const [addName, setAddName] = useState("");
  const { token } = useAuth();
  const { createPlaylistMutation } = usePlaylists();

  if (!token) {
    return null;
  }

  const filtered = _.pickBy(playlists, (playlist) =>
    playlist.name.toLowerCase().includes(searchFilter.toLowerCase()),
  );

  const sorted = _.sortBy(filtered, (p) => p.name);

  const playlistlist = sorted.map((p) => {
    return <Playlist key={p.id} playlist={p} />;
  });

  // TODO: When creating a new playlist, redirect to that playlist
  function addPlaylist(name: string) {
    if (!name) {
      return;
    }

    createPlaylistMutation.mutate(addName);
  }

  function handleAdd() {
    addPlaylist(addName);
    setAddName("");
  }

  // TODO: Fix the overflow on this
  return (
    <div className="h-full max-h-full w-full overflow-auto">
      <div className="flex px-2 pt-2">
        <input
          className="bg-neutral-400/10 text-white w-full text-center"
          type="text"
          placeholder="Search"
          value={searchFilter}
          onChange={(e) => {
            setSearchFilter(e.target.value);
          }}
        />
      </div>

      <div className="py-2">{playlistlist}</div>

      <div className="flex px-2 pb-2">
        <input
          className="bg-neutral-400/10 flex inline-block w-full items-center text-white text-center"
          type="text"
          placeholder="Playlist Name"
          value={addName}
          onChange={(e) => {
            setAddName(e.target.value);
          }}
        />
        <button type="button" className="text-white pl-2" onClick={handleAdd}>
          <Plus size={18} />
        </button>
      </div>
    </div>
  );
}
