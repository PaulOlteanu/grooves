export type ApiToken = string;

export type Song = {
  name: string;
  spotify_id: string;
};

export type PlaylistElement = {
  name: string;
  songs: Song[];
};

export type Playlist = {
  name: string;
  elements: PlaylistElement[];
  id: number;
};

export enum DraggableType {
  Element,
  Song,
  SearchResult,
}
