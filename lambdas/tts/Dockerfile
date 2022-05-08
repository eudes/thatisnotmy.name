FROM alpine
# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . /usr/src/app/

# Install production dependencies and build a release artifact.
#RUN cargo install --path .

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080
RUN cat /proc/cpuinfo
RUN ls /usr/src/app/
RUN chmod +x /usr/src/app/start.sh && chmod +x /usr/src/app/tts-lambda

# Run the web service on container startup.
CMD ["/usr/src/app/start.sh"]
