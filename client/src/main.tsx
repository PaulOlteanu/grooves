import React from "react";
import ReactDOM from "react-dom/client";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import "./index.css";
import ErrorPage from "./error-page";

import Root from "./routes/root";
import { QueryClient, QueryClientProvider } from "react-query";
import PlaylistsPage from "./routes/playlists/playlistsPage";
import PlaylistPage from "./routes/playlists/playlistPage";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Root />,
    errorElement: <ErrorPage />,
    children: [
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
    ],
  },
]);

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <RouterProvider router={router} />
    </QueryClientProvider>
  </React.StrictMode>
);
