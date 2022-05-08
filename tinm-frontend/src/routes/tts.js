import {Link, useParams} from "react-router-dom";
import {useEffect, useRef, useState} from "react";
import {CachingApi} from "../CachingApi";
import log from 'loglevel';

export function TTS() {
  const {language, country, word} = useParams();
  const [isLoaded, setIsLoaded] = useState(false);
  const [error, setError] = useState(null);
  const [audioUrl, setAudioUrl] = useState('');
  const audioRef = useRef(new Audio(audioUrl));

  const api = CachingApi.instance();

  // Note: the empty deps array [] means
  // this useEffect will run once
  // similar to componentDidMount()
  useEffect(() => {
    if(!isLoaded) {
      api.fetch('https://tts-hzc3i5dapa-ew.a.run.app/tts', {
        method: 'POST', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'force-cache', // *default, no-cache, reload, force-cache, only-if-cached
        headers: {
          'Accept': 'application/json',
          'Content-Type': 'application/json',
        },
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
        body: JSON.stringify({
          language,
          word,
          countryCode: country.toUpperCase(),
        }) // body data type must match "Content-Type" header
      })
        .then(
          (result) => {
            if (!isLoaded) {
              setIsLoaded(true);
              const url = URL.createObjectURL(b64toBlob(result.audioContent, 'audio/mp3'));
              setAudioUrl(url);
              setError(null);
            }
          },
          // Note: it's important to handle errors here
          // instead of a catch() block so that we don't swallow
          // exceptions from actual bugs in components.
          (error) => {
            log.debug(error);
            setError(error.toString());
          }
        )
        .then(() => {
          audioRef.current.src = audioUrl;
        })
    }
  }, [])

  //https://stackoverflow.com/questions/16245767/creating-a-blob-from-a-base64-string-in-javascript
  const b64toBlob = (b64Data, contentType = '', sliceSize = 512) => {
    const byteCharacters = atob(b64Data);
    const byteArrays = [];

    for (let offset = 0; offset < byteCharacters.length; offset += sliceSize) {
      const slice = byteCharacters.slice(offset, offset + sliceSize);

      const byteNumbers = new Array(slice.length);
      for (let i = 0; i < slice.length; i++) {
        byteNumbers[i] = slice.charCodeAt(i);
      }

      const byteArray = new Uint8Array(byteNumbers);
      byteArrays.push(byteArray);
    }

    return new Blob(byteArrays, {type: contentType});
  }

  if (error) {
    return (
      <div>
        <p>Error</p>
      </div>
    )
  }
  if (!isLoaded) {
    return <div>Loading...</div>;
  } else {
    return (
      <>
        <main>
          <h2>Welcome to the TTS!</h2>
          <p>Success!</p>
          <p>{language}, {country}, {word}</p>
          <audio controls ref={audioRef} src={audioUrl} >
            Oops
          </audio>
        </main>
        <nav>
          <Link to="/">Home</Link>
        </nav>
      </>
    );
  }
}
