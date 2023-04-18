import { useQueryClient } from "react-query";
import { NavLink, Link, useNavigate } from "react-router-dom";
import { useAuth } from "../contexts/auth";

function LogoutButton() {
  const navigate = useNavigate();

  const { clearToken } = useAuth();
  const queryClient = useQueryClient();

  function handleLogout() {
    clearToken();
    queryClient.removeQueries();
    navigate("/");
  }

  return (
    <button
      className="bg-emerald-500 hover:bg-emerald-700 font-bold rounded px-2"
      type="button"
      onClick={handleLogout}
    >
      Logout
    </button>
  );
}

// TODO: Hamburger menu
export default function Navbar() {
  const { token } = useAuth();

  // TODO: Make this better
  const activeClasses = "inline-block text-white font-bold no-underline";
  const inactiveClasses =
    "inline-block no-underline hover:text-gray-200 hover:text-underline";

  return (
    <nav className="w-full mx-auto flex flex-wrap items-center justify-between">
      <Link
        className="text-white no-underline hover:no-underline font-extrabold text-xl"
        to="/"
      >
        Grooves
      </Link>

      <div className="flex-grow items-center w-auto block">
        <ul className="list-reset flex justify-end flex-1 items-center">
          <li className="mr-3">
            <NavLink
              className={({ isActive }) =>
                isActive ? activeClasses : inactiveClasses
              }
              to="/"
            >
              Home
            </NavLink>
          </li>
          <li className="mr-3">
            <NavLink
              className={({ isActive }) =>
                isActive ? activeClasses : inactiveClasses
              }
              to="/playlists"
            >
              Playlists
            </NavLink>
          </li>
          <li className="mr-3 last:mr-0">
            <NavLink
              className={({ isActive }) =>
                isActive ? activeClasses : inactiveClasses
              }
              to="/player"
            >
              Player
            </NavLink>
          </li>
          <li>{token ? <LogoutButton /> : null}</li>
        </ul>
      </div>
    </nav>
  );
}
