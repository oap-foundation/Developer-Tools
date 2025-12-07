FROM php:8.2-cli-alpine

# Install extensions
RUN apk add --no-cache \
    libsodium-dev \
    libzip-dev \
    zip \
    && docker-php-ext-install sodium zip

# Install Composer
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# Set working directory
WORKDIR /app

# Copy project files
# We mimic the local structure so relative paths in composer.json work
# Local: Developer Tools/oap-echo-bot -> ../../Language Bindings
# Docker: /app/Developer Tools/oap-echo-bot -> ../../Language Bindings

COPY ["Language Bindings", "/app/Language Bindings/"]
COPY ["Developer Tools/oap-echo-bot", "/app/Developer Tools/oap-echo-bot/"]

# Install dependencies
WORKDIR "/app/Developer Tools/oap-echo-bot"
RUN composer install --no-dev --optimize-autoloader

# Generate Identity if not exists (handled by entrypoint or manual volume mount)
# For this image, we assume keys are mounted or generated at runtime.

# Run Bot
CMD ["php", "scripts/run_bot.php"]
