import { useMutation, useQuery, useQueryClient } from "react-query";
import api from "../api";
import { useAuth } from "../contexts/auth";

export default function usePlaylists() {
  const queryClient = useQueryClient();
  const { token } = useAuth();

  const {
    isLoading,
    isError,
    data: playlists,
  } = useQuery(
    "playlists",
    async () => {
      if (!token) {
        return Promise.reject();
      } else {
        return await api.getPlaylists(token);
      }
    },
    { retry: false }
  );

  const createPlaylistMutation = useMutation(
    (name: string) => {
      if (!token) {
        return Promise.reject();
      } else {
        return api.createPlaylist(name, token);
      }
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
