CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Align with blockchain enums exactly
CREATE TYPE transaction_status AS ENUM ('pending', 'confirmed', 'failed');
CREATE TYPE mint_request_status AS ENUM ('pending', 'completed', 'rejected');
CREATE TYPE burn_request_status AS ENUM ('pending', 'reserved', 'completed', 'rejected');
CREATE TYPE kyc_status AS ENUM ('notsubmitted', 'pending', 'verified', 'rejected');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    wallet_address VARCHAR(66) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_address VARCHAR(66) NOT NULL,
    to_address VARCHAR(66) NOT NULL,
    amount VARCHAR(40) NOT NULL,
    fee VARCHAR(40) NOT NULL,
    transaction_hash VARCHAR(66),
    block_number BIGINT,
    status transaction_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE mint_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    wallet_address VARCHAR(66) NOT NULL,
    amount VARCHAR(40) NOT NULL,
    bank_reference VARCHAR(256) NOT NULL,
    status mint_request_status NOT NULL DEFAULT 'pending',
    chain_request_id BIGINT,
    approved_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE burn_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    wallet_address VARCHAR(66) NOT NULL,
    amount VARCHAR(40) NOT NULL,
    bank_account VARCHAR(256) NOT NULL,
    status burn_request_status NOT NULL DEFAULT 'pending',
    chain_request_id BIGINT,
    approved_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE kyc_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    wallet_address VARCHAR(66) NOT NULL,
    document_hash VARCHAR(128) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    date_of_birth TIMESTAMP WITH TIME ZONE NOT NULL,
    country VARCHAR(2) NOT NULL,
    status kyc_status NOT NULL DEFAULT 'notsubmitted',
    verified_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_wallet ON users(wallet_address);
CREATE INDEX idx_transactions_from ON transactions(from_address);
CREATE INDEX idx_transactions_to ON transactions(to_address);
CREATE INDEX idx_transactions_hash ON transactions(transaction_hash);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_created ON transactions(created_at DESC);
CREATE INDEX idx_mint_requests_user ON mint_requests(user_id);
CREATE INDEX idx_mint_requests_wallet ON mint_requests(wallet_address);
CREATE INDEX idx_mint_requests_status ON mint_requests(status);
CREATE INDEX idx_mint_requests_created ON mint_requests(created_at DESC);
CREATE INDEX idx_burn_requests_user ON burn_requests(user_id);
CREATE INDEX idx_burn_requests_wallet ON burn_requests(wallet_address);
CREATE INDEX idx_burn_requests_status ON burn_requests(status);
CREATE INDEX idx_burn_requests_created ON burn_requests(created_at DESC);
CREATE INDEX idx_kyc_user ON kyc_submissions(user_id);
CREATE INDEX idx_kyc_wallet ON kyc_submissions(wallet_address);
CREATE INDEX idx_kyc_status ON kyc_submissions(status);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_transactions_updated_at BEFORE UPDATE ON transactions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_mint_requests_updated_at BEFORE UPDATE ON mint_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_burn_requests_updated_at BEFORE UPDATE ON burn_requests
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_kyc_submissions_updated_at BEFORE UPDATE ON kyc_submissions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
