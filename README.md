# Morph URL
Free and Lightweight search and indexing API to create perfect search result for your business. Morphsearch helps you search related keywords for user and your business inquires. ✓ Check it out!

## How to
The current API development will let you search related keywords in a list of Array:

## APIs

### 1. Search Related keywords: `https://morphsearch.herokuapp.com/api/v1`

```
Request: 
{
  word (String): ... (The keyword you want to search),
  word_array: ... (The list of keywords available that you want to index)
}

Response (Array):
[
  {
    word: ...
    similarity: ... (in percentage)
  }
]
```

```
Example:
![image](https://user-images.githubusercontent.com/53996155/160053706-4f6ec823-a1ab-42da-a376-445fd5e7ee83.png)

```
