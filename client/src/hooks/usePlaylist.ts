import { useMutation, useQuery, useQueryClient } from "react-query";
import { useAuth } from "../contexts/auth";
import { Playlist } from "../types";

export default function usePlaylist(id: number) {
  const queryClient = useQueryClient();
  const { apiClient } = useAuth();

  const {
    isLoading,
    isError,
    data: playlist,
  } = useQuery(
    ["playlist", id],
    async () => {
      if (!apiClient) {
        return Promise.reject();
      } else {
        return await apiClient.getPlaylist(id);
      }
    },
    { retry: false, refetchOnWindowFocus: false },
  );

  const updatePlaylistMutation = useMutation(
    (playlist: Playlist) => {
      if (!apiClient) {
        return Promise.reject();
      } else {
        return apiClient.updatePlaylist(playlist);
      }
    },
    {
      onSuccess: (playlist) => {
        void queryClient.invalidateQueries({
          queryKey: ["playlist", playlist.id],
        });
      },

    },
  );

  const deletePlaylistMutation = useMutation(
    (playlist: Playlist) => {
      if (!apiClient) {
        return Promise.reject();
      } else {
        void apiClient.deletePlaylist(playlist.id);
        return new Promise<number>((resolve) => resolve(playlist.id));
      }
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
    },
  );

  return {
    isLoading,
    isError,
    playlist,
    updatePlaylistMutation,
    deletePlaylistMutation,
  };
}
