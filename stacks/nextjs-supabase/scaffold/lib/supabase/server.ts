// Server Supabase client factory — use in Server Components, Server Actions, and Route Handlers.
import { createServerClient as _createServerClient } from "@supabase/ssr";
import { cookies } from "next/headers";
import { getSupabasePublicConfig } from "./config";

export async function createServerClient() {
  const cookieStore = await cookies();
  const { url, anonKey } = getSupabasePublicConfig();

  return _createServerClient(url, anonKey, {
    cookies: {
      getAll() {
        return cookieStore.getAll();
      },
      setAll(cookiesToSet) {
        for (const { name, value, options } of cookiesToSet) {
          try {
            cookieStore.set(name, value, options);
          } catch {
            // setAll can be called from a Server Component where cookies
            // are read-only. The middleware will refresh the session instead.
          }
        }
      },
    },
  });
}
