-- CreateSchema
CREATE SCHEMA IF NOT EXISTS "public";

-- CreateTable
CREATE TABLE "users" (
    "id" UUID NOT NULL,
    "email" TEXT NOT NULL,
    "name" TEXT,
    "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" TIMESTAMP(3) NOT NULL,

    CONSTRAINT "users_pkey" PRIMARY KEY ("id")
);

-- CreateIndex
CREATE UNIQUE INDEX "users_email_key" ON "users"("email");

-- Supabase Auth owns profile identity. Expose only the authenticated user's
-- own profile through the Data API; no anonymous or cross-user policy exists.
ALTER TABLE "public"."users" ENABLE ROW LEVEL SECURITY;

CREATE POLICY "users_select_own_profile"
    ON "public"."users"
    FOR SELECT
    TO authenticated
    USING ((SELECT auth.uid()) = "id");

CREATE POLICY "users_insert_own_profile"
    ON "public"."users"
    FOR INSERT
    TO authenticated
    WITH CHECK ((SELECT auth.uid()) = "id");

CREATE POLICY "users_update_own_profile"
    ON "public"."users"
    FOR UPDATE
    TO authenticated
    USING ((SELECT auth.uid()) = "id")
    WITH CHECK ((SELECT auth.uid()) = "id");
