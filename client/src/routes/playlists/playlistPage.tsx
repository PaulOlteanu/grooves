import { useParams } from "react-router-dom";
import { default as PlaylistComponent } from "../../components/playlist";
import Search from "../../components/search";
import { useAuth } from "../../contexts/auth";
import usePlaylist from "../../hooks/usePlaylist";
import { PlaylistElement } from "../../types";
import { addElement } from "../../util/playlists";

function Page({ playlistId }: { playlistId: number }) {
  const { playlist, updatePlaylistMutation } = usePlaylist(playlistId);

  if (playlist) {
    const addAlbum = (newElement: PlaylistElement) => {
      const updatedPlaylist = addElement(playlist, newElement);
      updatePlaylistMutation.mutate(updatedPlaylist);
    };

    return (
      <div className="grid grid-cols-2 gap-[2%] min-w-full h-full max-h-full min-h-full">
        <div className="bg-neutral-400/10 max-h-full rounded-md flex-grow min-h-0">
          <PlaylistComponent playlist={playlist} />
        </div>

        <div className="bg-neutral-400/10 max-h-full rounded-md flex-grow min-h-0">
          <Search addAlbum={addAlbum} />
        </div>
      </div>
    );
  }

  return null;
}

export default function PlaylistPage() {
  const { playlistId } = useParams();
  const { token } = useAuth();

  const id = parseInt(playlistId || "");
  if (!Number.isNaN(id) && token) {
    return <Page playlistId={id} />;
  }

  return null;
}
