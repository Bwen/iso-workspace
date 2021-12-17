
Run preview completely standalone:
```bash
npm run build
docker run --name svelte-preview -p 9090:80 --rm -v $(pwd)/build:/usr/share/nginx/html:ro -d nginx
```
