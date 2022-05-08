import log from 'loglevel';

class CacheableObject {
  constructor(obj) {
    for (const [key, value] of Object.entries(obj)) {
      this[key] = value
    }
  }

  hash() {
    let hash = 0
    for (let i = 0; i < this.length; i++) {
      const code = this.charCodeAt(i)
      hash = ((hash << 5) - hash) + code
      hash = hash & hash // Convert to 32bit integer
    }
    return hash
  }

}

export class CachingApi {

  #requests
  #responses

  constructor() {
    this.#requests = {}
    this.#responses = {}
  }

  static instance() {
    return instance
  }

  fetch(url, options) {
    log.debug("Fetch requested")
    const request = {url, options}
    const cacheableObject = new CacheableObject(request)
    const hash = cacheableObject.hash()

    if (this.#requests[hash]) {
      log.debug("Request cached")

      const response = this.#responses[hash];
      if (response) {
        log.debug("Response cached")
        if (typeof response.then === 'function') {
          log.debug("Response is promise")
          return response
        }
        log.debug("Response is response")
        return Promise.resolve(response)
      }
      log.debug("Response is empty")
      return Promise.reject('Cached request exists, but no response. Unexpected...')
    }

    this.#requests[hash] = true

    log.debug("Fetching")
    const responsePromise = fetch(url, options)
      .then(response => {
        log.debug("Saving response as response")
        const json = response.json()
        this.#responses[hash] = json
        return json
      })

    if (!this.#responses[hash]) {
      log.debug("Saving response as promise")
      this.#responses[hash] = responsePromise
    }

    log.debug("Response is served straight")
    return responsePromise
  }

}

const instance = new CachingApi()
