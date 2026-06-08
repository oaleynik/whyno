from mitmproxy import http
from mitmproxy import ctx

class InterceptAndRespond:
    def request(self, flow: http.HTTPFlow) -> None:
        # Dump the request details to the mitmproxy log
        ctx.log.info(f"--- Intercepted Request ---")
        ctx.log.info(f"URL: {flow.request.method} {flow.request.url}")
        ctx.log.info(f"Headers: {flow.request.headers}")
        ctx.log.info(f"Content: {flow.request.content}")
        ctx.log.info(f"---------------------------")
        
        # Instantly respond with 200 OK and block the request from going to the actual server
        flow.response = http.Response.make(
            200,  # status code
            b"Intercepted and returned 200 OK",  # content
            {"Content-Type": "text/plain"}  # headers
        )

addons = [
    InterceptAndRespond()
]
