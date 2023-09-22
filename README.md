# NotitiaRust

This project was created to compare Rust and C++.

Initially I wrote this program in Rust and later the same in C++ ([NotitiaCpp](https://github.com/asestacom/NotitiaCpp)).

After a little refactoring, the code content is a little different, keeping some function and struct names but following the same logic to read big files, search words and creating the same output.

Another thing made on purpose is to avoid using objects allocated in heap directly (Containers will allocate in heap by themselves, including strings).

## Application

- [IMDB data](https://datasets.imdbws.com/)
- [The Movies Dataset (rounakabanik) - kaggle.com [TMDB with IMDB ids]](https://www.kaggle.com/datasets/rounakbanik/the-movies-dataset)
- [Movies Daily Update Dataset - TMDB (akshaypawar7) archive](https://www.kaggle.com/datasets/akshaypawar7/millions-of-movies)
- [Justwatch filtered](https://apis.justwatch.com/graphql)

## Configuration

### filter_node_format

> A list of valid format for input.

### working_file

> File to get updated, it must be a valid json file or non existant/blank file.

### output_file

> File with the result. Once the result is what you expect, maybe you can copied into working_file.

> Also you can use the very same file for working_file or for output_file.

### updating
    
> It will be true if you want to update o generate a new content.

### raw_list_movie_file

> A list of node names to identify.

## Result

If any of following values is set to true, that object won't be modified:
- imdb_confirmed
- justwatch_confirmed
- tmdb_confirmed

## Dependencies

serde
csv

# Tests

cargo test
