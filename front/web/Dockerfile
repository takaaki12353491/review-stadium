FROM node:16

# Set the working directory in the container to /app
WORKDIR /app

# Install pnpm globally in the image
RUN npm install -g pnpm@8.5

# Copy the root directory's package.json, pnpm-lock.yaml and pnpm-workspace.yaml to utilize Docker cache
COPY .npmrc ./package.json ./pnpm-lock.yaml ./pnpm-workspace.yaml ./

# Copy the common/nodejs directory
COPY ./common/nodejs ./common/nodejs

# Copy the front/web directory
COPY ./front/web ./front/web

# Install the project dependencies
RUN pnpm install --frozen-lockfile

# Change working directory to front/web
WORKDIR /app/front/web

# Build the application
RUN pnpm build

# Expose port 3000 in the container
EXPOSE 3000

# Define the command that should be executed when the container starts
CMD ["pnpm", "start"]