DO $$
BEGIN
    -- Drop the table if it exists
    DROP TABLE IF EXISTS big_data;

    -- Create the table
    CREATE TABLE big_data (
        id SERIAL PRIMARY KEY,
        username TEXT NOT NULL,
        email TEXT NOT NULL,
        age INT,
        city TEXT,
        created_at TIMESTAMP DEFAULT NOW(),
        bio TEXT
    );

    -- Insert 1 million rows of random data
    -- ⚠️ Change the upper bound if you want more or less data
    INSERT INTO big_data (username, email, age, city, created_at, bio)
    SELECT
        'user_' || g AS username,
        'user_' || g || '@example.com' AS email,
        (random() * 60 + 18)::int AS age,
        (ARRAY['New York','London','Berlin','Tokyo','Paris','Madrid','Sydney'])[floor(random()*7)] AS city,
        NOW() - (random() * (365*3)) * INTERVAL '1 day',
        md5(random()::text) || md5(random()::text)
    FROM generate_series(1, 1000000) g;  -- adjust row count here

END $$;