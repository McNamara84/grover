FROM mcr.microsoft.com/playwright:v1.65.1-noble
WORKDIR /tests
COPY package.json package-lock.json ./
RUN npm ci --no-audit --no-fund
COPY playwright.config.ts tsconfig.json .oxlintrc.json ./
COPY tests tests
ENTRYPOINT ["./node_modules/.bin/playwright", "test"]
