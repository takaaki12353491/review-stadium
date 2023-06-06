FROM node:18.16

# Set the working directory in the container to /app
WORKDIR /app

# Install pnpm globally in the image
RUN npm install -g pnpm

# Copy the root directory's package.json, pnpm-lock.yaml and pnpm-workspace.yaml to utilize Docker cache
COPY ./package.json ./pnpm-lock.yaml ./pnpm-workspace.yaml ./

# Copy the front/web directory
COPY ./front/web ./front/web

# Change working directory to front/web
WORKDIR /app/front/web

# Install the project dependencies
RUN pnpm install --frozen-lockfile

# Build the application
RUN pnpm run build

# Expose port 3000 in the container
EXPOSE 3000

# Define the command that should be executed when the container starts
CMD ["pnpm", "start"]