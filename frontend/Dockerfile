# Multi-stage build for Svelte frontend
FROM node:22-alpine AS builder

# Set working directory
WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm ci --only=production

# Copy source code
COPY . .

# Build the application
RUN npm run build

# Production stage - simple static file server
FROM node:22-alpine

# Install a simple HTTP server
RUN npm install -g serve

# Copy built files
COPY --from=builder /app/build /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start the static file server
CMD ["serve", "-s", "/usr/share/nginx/html", "-l", "80"] 