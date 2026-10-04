import { Client } from "pg";

const client = new Client({ connectionString: process.env.DATABASE_URL });
await client.connect();

try {
  await client.query("CREATE ROLE anon NOLOGIN");
  await client.query("CREATE ROLE authenticated NOLOGIN");
  await client.query("CREATE SCHEMA auth");
  await client.query(`
    CREATE FUNCTION auth.uid() RETURNS uuid
    LANGUAGE sql STABLE
    AS $$
      SELECT NULLIF(current_setting('request.jwt.claim.sub', true), '')::uuid
    $$
  `);
  await client.query("GRANT USAGE ON SCHEMA auth TO anon, authenticated");
  await client.query("GRANT USAGE ON SCHEMA public TO anon, authenticated");
} finally {
  await client.end();
}
