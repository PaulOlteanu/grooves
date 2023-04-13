import { useMutation, useQuery, useQueryClient } from "react-query";
import api from "../api";
import { useAuth } from "../contexts/auth";
import { Playlist } from "../types";

export default function usePlaylist(id: number) {
  const queryClient = useQueryClient();
  const { token } = useAuth();

  const {
    isLoading,
    isError,
    data: playlist,
  } = useQuery(
    ["playlist", id],
    async () => {
      if (!token) {
        return Promise.reject();
      } else {
        return await api.getPlaylist(id, token);
      }
    },
    { retry: false }
  );

  const updatePlaylistMutation = useMutation(
    (playlist: Playlist) => {
      if (!token) {
        return Promise.reject();
      } else {
        return api.updatePlaylist(playlist, token);
      }
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
      if (!token) {
        return Promise.reject();
      } else {
        void api.deletePlaylist(playlist.id, token);
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
