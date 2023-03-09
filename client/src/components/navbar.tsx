import { NavLink, Link, useNavigate } from "react-router-dom";
// import { useAuth } from "../auth/context";

function LogoutButton() {
  const navigate = useNavigate();

  // const { setApiToken } = useAuth();

  function handleLogout() {
    localStorage.clear();
    // setApiToken(null);
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
  // const { apiToken } = useAuth();

  // TODO: Make this better
  // const baseClasses = "inline-block py-2 px-4"
  const activeClasses = "inline-block text-white font-bold no-underline";
  const inactiveClasses =
    "inline-block no-underline hover:text-gray-200 hover:text-underline";

  return (
    <nav className="w-full mx-auto flex flex-wrap items-center justify-between">
      <Link
        className="text-white no-underline hover:no-underline font-extrabold text-xl"
        to="/"
      >
        Phonos
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
        </ul>
      </div>

      {/* {apiToken ? <LogoutButton /> : null} */}
    </nav>
  );
}
