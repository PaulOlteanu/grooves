import axios, { AxiosInstance } from "axios";
import { ApiToken, Playlist, Song } from "./types";

export type ApiError = {
  message: string;
};

export function isApiError(x: unknown): x is ApiError {
  return (x as ApiError).message !== undefined;
}

export type ApiResponse<T> = T | ApiError;

export type SongSearchResult = {
  name: string;
  spotify_id: string;
  image_url: string;
};

export type AlbumSearchResult = {
  name: string;
  spotify_id: string;
  image_url: string;
  songs: Song[];
};

export type SearchResults = {
  songs: SongSearchResult[];
  albums: AlbumSearchResult[];
};

export default class ApiClient {
  axiosClient: AxiosInstance;

  constructor(token: ApiToken) {
    this.axiosClient = axios.create({
      baseURL: import.meta.env.VITE_API_URL,
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
  }

  async getPlaylists() {
    const { data } = await this.axiosClient.get<Playlist[]>("/playlists");
    return data;
  }

  // TODO: Make this take a playlist
  async createPlaylist(name: string) {
    const { data } = await this.axiosClient.post<Playlist>("/playlists", {
      name,
      elements: [],
    });

    return data;
  }

  async getPlaylist(playlistId: number) {
    const { data } = await this.axiosClient.get<Playlist>(
      `/playlists/${playlistId}`
    );
    return data;
  }

  async updatePlaylist(playlist: Playlist) {
    const { data } = await this.axiosClient.put<Playlist>(
      `/playlists/${playlist.id}`,
      playlist
    );

    return data;
  }

  async deletePlaylist(playlistId: number) {
    return this.axiosClient.delete(`/playlists/${playlistId}`, {});
  }

  async search(query: string) {
    const { data } = await this.axiosClient.get<SearchResults>(
      `/spotify/search?q=${query}`
    );

    return data;
  }

  async albumSongs(albumId: string) {
    const { data } = await this.axiosClient.get<Song[]>(
      `/spotify/album_songs/${albumId}`
    );

    return data;
  }

  async sendPlayerCommand(command: object) {
    await this.axiosClient.post("/player", command);
  }
}
