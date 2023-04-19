import { isRouteErrorResponse, useRouteError } from "react-router-dom";
import { isApiError } from "./api";

export default function ErrorPage() {
  const error = useRouteError();
  console.error(error);

  // TODO: Make this better
  if (isRouteErrorResponse(error)) {
    return (
      <div id="error-page" className="h-screen text-white bg-neutral-800">
        <h1>Oops!</h1>
        <p>
          {error.status} {error.statusText}
        </p>
      </div>
    );
  } else if (isApiError(error)) {
    return (
      <div id="error-page" className="h-screen text-white bg-neutral-800">
        <h1>Error</h1>
        <p>{error.message}</p>
      </div>
    );
  } else {
    return (
      <div id="error-page" className="h-screen text-white bg-neutral-800">
        <h1>Error</h1>
      </div>
    );
  }
}
