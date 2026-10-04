// Browser Supabase client factory — use in Client Components only.
import { createBrowserClient as _createBrowserClient } from "@supabase/ssr";
import { getSupabasePublicConfig } from "./config";

export function createBrowserClient() {
  const { url, anonKey } = getSupabasePublicConfig();
  return _createBrowserClient(url, anonKey);
}
