import { useMutation, useQuery, useQueryClient } from "react-query";
import { useAuth } from "../contexts/auth";

export default function usePlaylists() {
  const queryClient = useQueryClient();
  const { apiClient } = useAuth();

  const {
    isLoading,
    isError,
    data: playlists,
  } = useQuery(
    "playlists",
    async () => {
      if (!apiClient) {
        return Promise.reject();
      } else {
        return await apiClient.getPlaylists();
      }
    },
    { retry: false }
  );

  const createPlaylistMutation = useMutation(
    (name: string) => {
      if (!apiClient) {
        return Promise.reject();
      } else {
        return apiClient.createPlaylist(name);
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
