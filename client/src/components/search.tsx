import _ from "lodash";
import { Plus } from "@phosphor-icons/react";
import { useRef, useState } from "react";
import { SearchResults } from "../api";
import { PlaylistElement } from "../types";
import { useAuth } from "../contexts/auth";

function SearchResult({
  name,
  imageUrl,
  onAdd,
}: {
  name: string;
  imageUrl: string;
  onAdd: (event: React.MouseEvent<HTMLButtonElement>) => void;
}) {
  return (
    <div className="flex py-2">
      <img
        className="h-[32px] w-[32px]"
        height={32}
        width={32}
        src={imageUrl || undefined}
      />

      <div className="flex inline-block w-full items-center min-w-0 pl-2">
        <span className="whitespace-nowrap overflow-hidden text-ellipsis">
          <span>{name}</span>
        </span>
      </div>

      <button type="button" onClick={onAdd}>
        <Plus size={18} className="items-center" />
      </button>
    </div>
  );
}

export default function Search({
  addAlbum,
}: {
  addAlbum?: (newElement: PlaylistElement) => void;
}) {
  const [searchText, setSearchText] = useState("");
  const [results, setResults] = useState<SearchResults | null>(null);
  const { apiClient } = useAuth();

  const debouncedSearch = useRef(
    _.debounce(async (query) => {
      if (!query || query === "" || !apiClient) {
        setResults(null);
        return;
      }

      try {
        const res = await apiClient.search(query as string);
        setResults(res);
      } catch (e) {
        // TODO: Show an error
        console.error(e);
      }
    }, 500)
  ).current;

  if (!apiClient) {
    return null;
  }

  function handleSearch(e: React.ChangeEvent<HTMLInputElement>) {
    setSearchText(e.target.value);
    void debouncedSearch(e.target.value);
  }

  let albums = null;
  const songs = null;

  if (results) {
    albums = results.albums.map((a) => {
      const onAdd = () => {
        const run = async () => {
          if (addAlbum) {
            // TODO: don't add if there's an element with the same name already
            const songs = await apiClient.albumSongs(a.spotify_id);
            const newElement: PlaylistElement = { name: a.name, songs };
            addAlbum(newElement);
          }
        };

        void run();
      };

      return (
        <SearchResult
          name={a.name}
          imageUrl={a.image_url}
          key={a.spotify_id}
          onAdd={onAdd}
        />
      );
    });
    // songs = results.songs.map((s) => (
    //   <SearchResult result={s} key={s.spotify_id} />
    // ));
  }

  return (
    <div className="w-full h-full overflow-auto">
      <div className="text-center mb-4">
        <p>Spotify Search</p>

        <input
          className="bg-neutral-800 text-center"
          type="text"
          value={searchText}
          onChange={handleSearch}
        />
      </div>

      {results && (
        <div className="w-full h-full p-2">
          {albums && (
            <>
              <h4 className="text-center text-xl font-bold w-full">Albums</h4>
              <div className="p-4 divide-y">{albums}</div>
            </>
          )}
          {songs && (
            <>
              <h4 className="text-center text-xl font-bold w-full">Songs</h4>
              <div className="p-4 divide-y">{songs}</div>
            </>
          )}
        </div>
      )}
    </div>
  );
}
