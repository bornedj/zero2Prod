-- Add migration script here
CREATE TABLE subscriptions_tokens(
    subscription_token TEXT NOT NULL,
    subscription_token_id uuid NOT NULL
    REFERENCES subscriptions (id),
    PRIMARY KEY (subscription_token)
);
