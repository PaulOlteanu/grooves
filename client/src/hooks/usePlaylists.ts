import { useMutation, useQuery, useQueryClient } from "react-query";
import { useAuth } from "../contexts/auth";
import { Playlist } from "../types";

export default function usePlaylists() {
  const queryClient = useQueryClient();
  const { apiClient } = useAuth();

  const {
    isLoading,
    isError,
    data: playlists,
  } = useQuery(
    "playlists",
    async (): Promise<Playlist[]> => {
      if (!apiClient) {
        throw new Error("no api client");
      } else {
        return await apiClient.getPlaylists();
      }
    },
    { retry: false, refetchOnWindowFocus: false },
  );

  const createPlaylistMutation = useMutation(
    async (name: string) => {
      if (!apiClient) {
        throw new Error("no api client");
      } else {
        return await apiClient.createPlaylist(name);
      }
    },
    {
      onSuccess: () => {
        void queryClient.invalidateQueries({ queryKey: "playlists" });
      },
    },
  );

  return {
    isLoading,
    isError,
    playlists,
    createPlaylistMutation,
  };
}
