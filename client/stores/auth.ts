import * as SecureStore from "expo-secure-store";
import { create } from "zustand";

interface AuthState {
  bearerToken: string | null;
  isPending: boolean;
  setBearerToken: (token: string) => void;
  deleteBearerToken: () => void;
  initialize: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  bearerToken: null,
  isPending: true,

  setBearerToken: async (token: string) => {
    try {
      await SecureStore.setItemAsync("bearerToken", token);
      set({ bearerToken: token });
    } catch (error) {
      console.error("Failed to store token:", error);
    }
  },

  deleteBearerToken: async () => {
    try {
      await SecureStore.deleteItemAsync("bearerToken");
      set({ bearerToken: null });
    } catch (error) {
      console.error(error);
      throw error;
    }
  },

  initialize: async () => {
    set({ isPending: true });
    try {
      const bearerToken = await SecureStore.getItemAsync("bearerToken");
      set({ bearerToken, isPending: false });
    } catch (error) {
      console.error(error);
      set({ isPending: false });
    }
  },
}));
