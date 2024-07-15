import { Outlet } from "react-router-dom";
import Navbar from "../components/navbar";

export default function Root() {
  return (
    <div className="bg-neutral-900 text-neutral-100 h-screen min-h-screen max-h-screen w-screen min-w-screen max-w-screen flex flex-col">
      <div className="flex p-3">
        <Navbar />
      </div>

      <main className="p-3 mx-auto w-full min-w-full flex flex-grow min-h-0">
        <Outlet />
      </main>
    </div>
  );
}
