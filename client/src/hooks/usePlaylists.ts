import { useMutation, useQuery, useQueryClient } from "react-query";
import api from "../api";
import { Playlist } from "../types";

export default function usePlaylists() {
  const queryClient = useQueryClient();
  const {
    isLoading,
    isError,
    data: playlists,
  } = useQuery(
    "playlists",
    async () => {
      return await api.getPlaylists();
    },
    { retry: false }
  );

  const createPlaylistMutation = useMutation(
    (name: string) => {
      return api.createPlaylist(name);
    },
    {
      onSuccess: () => {
        void queryClient.invalidateQueries({ queryKey: "playlists" });
      },
    }
  );

  return {
    isLoading,
    isError,
    playlists,
    createPlaylistMutation,
  };
}
