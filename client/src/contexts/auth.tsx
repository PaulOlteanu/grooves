import { createContext, useContext, useState } from "react";
import ApiClient from "../api";
import type { ApiToken } from "../types";

type AuthContextType = {
  token: ApiToken | null;
  setToken: (token: string) => void;
  clearToken: () => void;
  apiClient: ApiClient | null;
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export default AuthContext;

export function useAuth() {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error("useAuth must be used within a Provider");
  }

  return context;
}

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [token, setToken] = useState<string | null>(() => {
    return localStorage.getItem("token");
  });

  const saveToken = (token: string) => {
    localStorage.setItem("token", token);
    return setToken(token);
  };

  const clearToken = () => {
    localStorage.removeItem("token");
    return setToken(null);
  };

  let apiClient = null;
  if (token) {
    apiClient = new ApiClient(token);
  }

  const value = { token, setToken: saveToken, clearToken, apiClient };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
