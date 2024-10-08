import { Outlet } from "react-router-dom";
import PlaylistSelector from "../../components/playlistSelector";
import usePlaylists from "../../hooks/usePlaylists";

export default function PlaylistsPage() {
  const { playlists } = usePlaylists();

  if (playlists) {
    return (
      <div className="grid grid-cols-[39%_59%] md:grid-cols-[18%_80%] gap-[2%] min-w-full">
        <div className="bg-neutral-600/10 rounded-md max-h-full flex-grow min-h-0">
          <PlaylistSelector playlists={playlists} />
        </div>

        <div className="max-h-full flex-grow min-h-0">
          <Outlet />
        </div>
      </div>
    );
  }

  // TODO: Handle loading
  return null;
}
