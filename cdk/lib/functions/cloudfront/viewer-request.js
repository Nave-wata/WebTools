function handler(event) {
  const request = event.request;
  const headers = request.headers;

  if (headers.host.value.endsWith('.cloudfront.net')) {
    return {
      statusCode: 403,
      statusDescription: "Forbidden",
      body: "Access denied",
    };
  }

  const uri = request.uri;

  // Check whether the URI is missing a file name.
  if (uri.endsWith("/")) {
    request.uri += "index.html";
  }
  // Check whether the URI is missing a file extension.
  else if (!uri.includes(".")) {
    request.uri += "/index.html";
  }

  return request;
}
