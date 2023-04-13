import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import "./index.css";
import ErrorPage from "./error-page";

import Root from "./routes/root";
import { QueryClient, QueryClientProvider } from "react-query";
import PlaylistsPage from "./routes/playlists/playlistsPage";
import PlaylistPage from "./routes/playlists/playlistPage";
import Player from "./routes/player";
import { AuthProvider } from "./contexts/auth";
import LoginCallback from "./routes/callback";
import Index from "./routes";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
      { path: "", element: <Index /> },
      {
        path: "callback",
        element: <LoginCallback />,
      },

      {
        path: "playlists",
        element: <PlaylistsPage />,
        children: [
          {
            path: ":playlistId",
            element: <PlaylistPage />,
          },
        ],
      },
      {
        path: "player",
        element: <Player />,
      },
    ],
  },
]);

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <AuthProvider>
      <QueryClientProvider client={queryClient}>
        <RouterProvider router={router} />
      </QueryClientProvider>
    </AuthProvider>
  </React.StrictMode>
);
