import { useMutation, useQuery, useQueryClient } from "react-query";
import api from "../api";
import { Playlist } from "../types";

export default function usePlaylist(id: number) {
  const queryClient = useQueryClient();
  const {
    isLoading,
    isError,
    data: playlist,
  } = useQuery(
    ["playlist", id],
    async () => {
      return await api.getPlaylist(id);
    },
    { retry: false }
  );

  const updatePlaylistMutation = useMutation(
    (playlist: Playlist) => {
      return api.updatePlaylist(playlist);
    },
    {
      onSuccess: (playlist) => {
        void queryClient.invalidateQueries({
          queryKey: ["playlist", playlist.id],
        });
      },
    }
  );

  const deletePlaylistMutation = useMutation(
    (playlist: Playlist) => {
      void api.deletePlaylist(playlist.id);
      return new Promise<number>((resolve) => resolve(playlist.id));
    },
    {
      onSuccess: (playlist_id) => {
        void queryClient.invalidateQueries({
          queryKey: "playlists",
        });
        void queryClient.invalidateQueries({
          queryKey: ["playlist", playlist_id],
        });
      },
    }
  );

  return {
    isLoading,
    isError,
    playlist,
    updatePlaylistMutation,
    deletePlaylistMutation,
  };
}
