# klassic-quote-api
한국 영화의 명대사를 보여주는 API 입니다.  
Axum + In memory DB

## API 정보
### URL : https://klassic-quote-api.mooo.com

## Web 정보
### URL : https://klassic-quote.vercel.app/

## Status page
### URL : https://ghghghko.github.io/upptime/

### endpoints
- https://klassic-quote-api.mooo.com/v1/quotes (GET)
```json
[
  {
    "id": 60,
    "quote": "정 마담",
    "author": "그 년한테 가는 거야? 그 년한테 가는 건 좋은데, 그 돈은 내려 놓구 가... 그 돈은 놓구 가!!!!"
  },
  {
    "id": 42,
    "quote": "고니",
    "author": "지랄하네. 어차피 좆같이 나가는 거 나도 세상 단맛, 쓴맛, 똥맛까지 다 먹어본 새끼야. 말빨 조지지마, 씨발."
  },
  {

...
```

- https://klassic-quote-api.mooo.com/v1/random-quote (GET)
```json
{
  "id": 59,
  "quote": "고니",
  "author": "난 딴 돈의 반만 가져가."
}
```

## TO-DO
- [x] API 문서 만들기  
- [ ] 타짜 이외에 영화 명대사 추가하기
- [ ] add renovate[bot]
