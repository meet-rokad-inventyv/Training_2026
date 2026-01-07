## Question 1 
***Write a SQL query to find the name and year of the movies. Return movie title, movie release year.***

#### SQL Query
````sql
SELECT mov_title,mov_year 
FROM dbo.movie;
````

#### Output:

| mov_title                | mov_year |
| :----------------------- | :------- |
| Vertigo                  | 1958     |
| The Innocents            | 1961     |
| Lawrence of Arabia       | 1962     |
| The Deer Hunter          | 1978     |
| Amadeus                  | 1984     |
| Blade Runner             | 1982     |
| Eyes Wide Shut           | 1999     |
| The Usual Suspects       | 1995     |
| Chinatown                | 1974     |
| Boogie Nights            | 1997     |
| Annie Hall               | 1977     |
| Princess Mononoke        | 1997     |
| The Shawshank Redemption | 1994     |
| American Beauty          | 1999     |
| Titanic                  | 1997     |
| Good Will Hunting        | 1997     |
| Deliverance              | 1972     |
| Trainspotting            | 1996     |
| The Prestige             | 2006     |
| Donnie Darko             | 2001     |
| Slumdog Millionaire      | 2008     |
| Aliens                   | 1986     |
| Beyond the Sea           | 2004     |
| Avatar                   | 2009     |
| Seven Samurai            | 1954     |
| Spirited Away            | 2001     |
| Back to the Future       | 1985     |
| Braveheart               | 1995     |

## Question 2
***Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.***
#### SQL Query
```sql
SELECT mov_year 
FROM dbo.movie 
WHERE mov_title = 'American Beauty';
```
#### Output:
| mov_year |
| :------- |
| 1999     |

## Question 3
Write a SQL query to find the movie that was released in 1999. Return movie title.
#### SQL Query
```sql
SELECT mov_title 
FROM dbo.movie 
WHERE mov_year = 1999;
```
#### Output
| mov_title       |
| :-------------- |
| Eyes Wide Shut  |
| American Beauty |


## Question 4
***Write a SQL query to find those movies, which were released before 1998. Return movie title.***
#### SQL Query
````sql
SELECT 
mov_title AS movie_title 
FROM 
movie 
WHERE mov_year < 1998;
````
#### Output
|                          |
| :----------------------- |
| movie_title              |
| Vertigo                  |
| The Innocents            |
| Lawrence of Arabia       |
| The Deer Hunter          |
| Amadeus                  |
| Blade Runner             |
| The Usual Suspects       |
| Chinatown                |
| Boogie Nights            |
| Annie Hall               |
| Princess Mononoke        |
| The Shawshank Redemption |
| Titanic                  |
| Good Will Hunting        |
| Deliverance              |
| Trainspotting            |
| Aliens                   |
| Seven Samurai            |
| Back to the Future       |
| Braveheart               |

## Question 5
Write a SQL query to find the name of all reviewers and movies together in a single list.
#### SQL Query
````sql
SELECT mov_title AS name FROM dbo.movie 
UNION
SELECT rev_name AS name FROM dbo.movie_reviewer where rev_name != '';
````
#### Output
| name                     |
| :----------------------- |
| Alec Shaw                |
| Aliens                   |
| Amadeus                  |
| American Beauty          |
| Annie Hall               |
| Avatar                   |
| Back to the Future       |
| Beyond the Sea           |
| Blade Runner             |
| Boogie Nights            |
| Brandt Sponseller        |
| Braveheart               |
| Chinatown                |
| Deliverance              |
| Donnie Darko             |
| Eyes Wide Shut           |
| Flagrant Baronessa       |
| Good Will Hunting        |
| Hannah Steele            |
| Jack Malvern             |
| Josh Cates               |
| Krug Stillo              |
| Lawrence of Arabia       |
| Mike Salvati             |
| Neal Wruck               |
| Paul Monks               |
| Princess Mononoke        |
| Richard Adams            |
| Righty Sock              |
| Sasha Goldshtein         |
| Scott LeBrun             |
| Seven Samurai            |
| Simon Wright             |
| Slumdog Millionaire      |
| Spirited Away            |
| The Deer Hunter          |
| The Innocents            |
| The Prestige             |
| The Shawshank Redemption |
| The Usual Suspects       |
| Titanic                  |
| Trainspotting            |
| Vertigo                  |
| Victor Woeltjen          |
| Vincent Cadena           |
| Wesley S. Walker         |

## Question 6
Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.
#### SQL Query
````sql
SELECT rev_name FROM movie_rating 
INNER JOIN
movie_reviewer ON movie_rating.rev_id = movie_reviewer.rev_id 
WHERE movie_rating.rev_stars >= 7;
````
#### Output
| rev_name           |
| :----------------- |
| Righty Sock        |
| Jack Malvern       |
| Flagrant Baronessa |
|                    |
| Simon Wright       |
| Mike Salvati       |
|                    |
| Sasha Goldshtein   |
| Righty Sock        |
| Hannah Steele      |
| Vincent Cadena     |
| Brandt Sponseller  |
| Victor Woeltjen    |
| Krug Stillo        |

## Question 7
Write a SQL query to find the movies without any rating. Return movie title.
#### SQL Query
````sql
SELECT mov_title FROM movie
INNER JOIN
movie_rating ON movie.mov_id = movie_rating.mov_id WHERE movie_rating.rev_stars  IS NULL;
````
#### Output
| mov_title     |
| :------------ |
| Chinatown     |
| Trainspotting |

## Question 8
Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.
#### SQL Query
````sql
SELECT mov_title 
FROM movie 
WHERE mov_id IN (905,907,917);
````
#### Output
| mov_title |
| :-------- |

## Question 9
Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.
#### SQL Query
````sql
SELECT mov_id,mov_title,mov_year FROM movie WHERE mov_title LIKE '%Boogie Nights%' ORDER BY mov_year ASC;
````
#### Output
| mov_id | mov_title     | mov_year |
| :----- | :------------ | :------- |
| 10     | Boogie Nights | 1997     |

## Question 10
***Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.***
#### SQL Query
````sql
SELECT 
act_id AS actor_id
FROM
actor
WHERE act_fname = 'Woody' AND acrt_lname = 'Allen';
````
#### Output
| actor_id |
| :------- |
| 11       |

## Question 11
***Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.***
#### SQL Query
````sql
SELECT 
* FROM actor WHERE act_id IN 
(SELECT act_id FROM movie_cast WHERE mov_id IN 
(SELECT mov_id FROM movie WHERE mov_title = 'Annie Hall'));
````
#### Output
| act_id | act_fname | acrt_lname | act_gender |
| :----- | :-------- | :--------- | :--------- |
| 11     | Woody     | Allen      | M          |
|        |           |            |            |

## Question 12
***Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.***
#### SQL Query
````sql
SELECT 
dir_fname AS director_fname, 
dir_lname AS director_lname
FROM director WHERE dir_id IN 
(SELECT dir_id FROM movie_direction WHERE mov_id IN
(SELECT mov_id FROM movie WHERE mov_title = 'Eyes Wide Shut'));
````
#### Output
| director_fname | director_lname |
| :------------- | :------------- |
| Stanley        | Kubrick        |

## Question 13
***Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.***
#### SQL Query
````sql
SELECT 
mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country 
FROM movie WHERE mov_rel_country <> 'UK';
````
#### Output
| mov_title     | mov_year | mov_time | mov_dt_rel | mov_rel_country |
| :------------ | :------- | :------- | :--------- | :-------------- |
| The Innocents | 1961     | 100      | 1962-02-19 | SW              |
| Annie Hall    | 1977     | 93       | 1977-04-20 | USA             |
| Seven Samurai | 1954     | 207      | 1954-04-26 | JP              |

## Question 14
***Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.***
#### SQL Query
````sql

SELECT 
mov_title,
mov_year,
mov_dt_rel,
(SELECT dir_fname 
 FROM director WHERE dir_id = 
		(SELECT dir_id FROM movie_direction WHERE mov_id = m.mov_id)) AS director_fname,
	    (SELECT dir_lname FROM director WHERE dir_id = (SELECT dir_id FROM movie_direction WHERE mov_id = m.mov_id)) AS director_lname,
	    (SELECT act_fname FROM actor WHERE act_id = (SELECT act_id FROM movie_cast WHERE mov_id = m.mov_id)) AS actor_fname,
	    (SELECT acrt_lname FROM actor WHERE act_id = (SELECT act_id FROM movie_cast WHERE mov_id = m.mov_id)) AS actor_lname FROM movie m WHERE mov_id IN 
	    (SELECT mov_id FROM movie_rating WHERE rev_id IN (SELECT rev_id FROM movie_reviewer WHERE rev_name = '')
);
````
#### Output
| mov_title         | mov_year | mov_dt_rel | director_fname | director_lname | actor_fname | actor_lname |
| :---------------- | :------- | :--------- | :------------- | :------------- | :---------- | :---------- |
| Blade Runner      | 1982     | 1982-09-09 | Ridley         | Scott          | Harrison    | Ford        |
| Princess Mononoke | 1997     | 2001-10-19 | Hayao          | Miyazaki       | Claire      | Danes       |

## Question 15
***Write a SQL query to find those movies directed by the director whose first name is Woddy and last name is Allen. Return movie title.***
#### SQL Query
```sql
	SELECT 
    m.mov_title,
    m.mov_year,
    m.mov_dt_rel,
    d.dir_fname,
    d.dir_lname,
    a.act_fname,
    a.acrt_lname
FROM movie m
JOIN movie_direction md 
    ON m.mov_id = md.mov_id
JOIN director d 
    ON md.dir_id = d.dir_id
JOIN movie_cast mc 
    ON m.mov_id = mc.mov_id
JOIN actor a 
    ON mc.act_id = a.act_id
WHERE m.mov_id IN (
    SELECT mr.mov_id
    FROM movie_rating mr
    JOIN movie_reviewer r 
        ON mr.rev_id = r.rev_id
    WHERE r.rev_name IS NULL
);
```
#### Output
| mov_title  |
| :--------- |
| Annie Hall |

## Question 16

***Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.***
#### SQL Query
```sql
SELECT DISTINCT mov_year
FROM movie
WHERE mov_id IN (
    SELECT mov_id
    FROM movie_rating
    WHERE rev_stars >= 3
)
ORDER BY mov_year ASC;
```
#### Output
| mov_year |
| :------- |
| 1954     |
| 1958     |
| 1961     |
| 1962     |
| 1977     |
| 1982     |
| 1986     |
| 1995     |
| 1997     |
| 1999     |
| 2001     |
| 2004     |
| 2008     |
| 2009     |

## Question 17
***Write a SQL query to search for movies that do not have any ratings. Return movie title.***
#### SQL Query
```sql
SELECT mov_title
FROM movie
WHERE mov_id NOT IN (
    SELECT mov_id
    FROM movie_rating WHERE num_o_ratings IS NOT NULL 
);
```
#### Output

| mov_title                |
| :----------------------- |
| The Deer Hunter          |
| Amadeus                  |
| Eyes Wide Shut           |
| Princess Mononoke        |
| The Shawshank Redemption |
| Deliverance              |
| The Prestige             |
| Avatar                   |
| Spirited Away            |
| Back to the Future       |
| Braveheart               |

## Question 18
***Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.***
#### SQL Query
````sql
SELECT 
rev_name FROM movie_reviewer 
WHERE rev_id  NOT IN(
SELECT rev_id FROM movie_rating
);
````
#### Output
| rev_name         |
| :--------------- |
| Alec Shaw        |
| Wesley S. Walker |

## Question 19
***Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.***
#### SQL Query
```sql

SELECT 
    mr.rev_name,
    m.mov_title,
    rt.rev_stars
FROM movie_rating rt
INNER JOIN movie m ON rt.mov_id = m.mov_id
INNER JOIN movie_reviewer mr ON rt.rev_id = mr.rev_id
WHERE rt.rev_stars IS NOT NULL 
    AND mr.rev_name IS NOT NULL 
    AND mr.rev_name != ''
ORDER BY mr.rev_name ASC, m.mov_title ASC, rt.rev_stars ASC;
```
#### Output

| rev_name           | mov_title           | rev_stars |
| :----------------- | :------------------ | :-------- |
| Brandt Sponseller  | Aliens              | 8.40      |
| Flagrant Baronessa | Lawrence of Arabia  | 8.30      |
| Hannah Steele      | Donnie Darko        | 8.10      |
| Jack Malvern       | The Innocents       | 7.90      |
| Josh Cates         | Good Will Hunting   | 4.00      |
| Krug Stillo        | Seven Samurai       | 7.70      |
| Mike Salvati       | Annie Hall          | 8.10      |
| Paul Monks         | Boogie Nights       | 3.00      |
| Richard Adams      | Beyond the Sea      | 6.70      |
| Righty Sock        | Titanic             | 7.70      |
| Righty Sock        | Vertigo             | 8.40      |
| Sasha Goldshtein   | American Beauty     | 7.00      |
| Simon Wright       | The Usual Suspects  | 8.60      |
| Victor Woeltjen    | Avatar              | 7.30      |
| Vincent Cadena     | Slumdog Millionaire | 8.00      |
|                    |                     |           |

## Question 20
***Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer’s name, movie title. Return reviewer’s name, movie title.***
#### SQL Query
```sql
SELECT DISTINCT
    (SELECT mr.rev_name
     FROM movie_reviewer mr
     WHERE mr.rev_id = r.rev_id) AS rev_name,
    (SELECT m.mov_title
     FROM movie m
     WHERE m.mov_id = r.mov_id) AS mov_title
FROM movie_rating r
WHERE r.rev_stars IS NOT NULL;
```
#### Output
| rev_name           | mov_title           |
| :----------------- | :------------------ |
|                    | Blade Runner        |
|                    | Princess Mononoke   |
| Brandt Sponseller  | Aliens              |
| Flagrant Baronessa | Lawrence of Arabia  |
| Hannah Steele      | Donnie Darko        |
| Jack Malvern       | The Innocents       |
| Josh Cates         | Good Will Hunting   |
| Krug Stillo        | Seven Samurai       |
| Mike Salvati       | Annie Hall          |
| Paul Monks         | Boogie Nights       |
| Richard Adams      | Beyond the Sea      |
| Righty Sock        | Titanic             |
| Righty Sock        | Vertigo             |
| Sasha Goldshtein   | American Beauty     |
| Simon Wright       | The Usual Suspects  |
| Victor Woeltjen    | Avatar              |
| Vincent Cadena     | Slumdog Millionaire |

## Question 21
***Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.***
#### SQL Query

```sql
SELECT m.mov_title,
       max(r.rev_stars) AS max_stars
FROM movie m, movie_rating r
WHERE m.mov_id = r.mov_id
  AND r.rev_stars = (
        SELECT MAX(rev_stars)
        FROM movie_rating
        WHERE rev_stars IS NOT NULL
      )
GROUP BY m.mov_title
ORDER BY m.mov_title ASC;
```
#### Output
| mov_title          | max_stars |
| :----------------- | :-------- |
| The Usual Suspects | 8.60      |

## Question 22
***Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name.***
#### SQL Query
```sql
SELECT rev_name FROM movie_reviewer WHERE rev_id IN(
    SELECT rev_id FROM movie_rating WHERE mov_id IN(
        SELECT mov_id FROM movie WHERE mov_title  = 'American Beauty'
    )
);
```
#### Output
| rev_name         |
| :--------------- |
| Sasha Goldshtein |
|                  |
## Question 23
***Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.***
#### SQL Query
```sql
select mov_title from movie where mov_id in (
    select mov_id from movie_rating where rev_id in (
        select rev_id from movie_reviewer where rev_name = 'Paul Monks'
    )
);
```
#### Output
| mov_title     |
| :------------ |
| Boogie Nights |

## Question 24
***Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.***
#### SQL Query
```sql
SELECT
    (SELECT mr.rev_name
     FROM movie_reviewer mr
     WHERE mr.rev_id = r.rev_id) AS reviewer_name,

    (SELECT m.mov_title
     FROM movie m
     WHERE m.mov_id = r.mov_id) AS movie_title,

    r.rev_stars
FROM movie_rating r
WHERE r.rev_stars = (
        SELECT MIN(rev_stars)
        FROM movie_rating
        WHERE rev_stars IS NOT NULL
      );
```
#### Output
| reviewer_name | movie_title   | rev_stars |
| :------------ | :------------ | :-------- |
| Paul Monks    | Boogie Nights | 3.00      |

## Question 25
***Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.***
#### SQL Query
```sql
SELECT mov_title FROM movie WHERE mov_id in (
    SELECT mov_id FROM movie_direction WHERE dir_id IN (
        SELECT dir_id FROM director WHERE dir_fname = 'James' AND dir_lname = 'Cameron'
    )
);
```
#### Output
| mov_title |
| :-------- |
| Titanic   |
| Aliens    |

## Question 26
***Write a query in SQL to find the movies in which one or more actors appeared in more than one film.***
#### SQL Query
```sql
SELECT DISTINCT m.mov_title
FROM movie m
WHERE m.mov_id IN (
    SELECT mc.mov_id
    FROM movie_cast mc
    WHERE mc.act_id IN (
        SELECT act_id
        FROM movie_cast
        GROUP BY act_id
        HAVING COUNT(DISTINCT mov_id) > 1
    )
);
```
#### Output
| mov_title       |
| :-------------- |
| American Beauty |
| Beyond the Sea  |

## Question 27
***Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.***
#### SQL Query
```sql 
SELECT rev_name FROM movie_reviewer mr 
JOIN movie_rating mrt ON mr.rev_id = mrt.rev_id WHERE rev_stars IS NULL;
```
#### Output
| rev_name     |
| :----------- |
| Neal Wruck   |
| Scott LeBrun |

## Question 28
***Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.***
#### SQL Query
```sql
SELECT
    a.act_fname,
    a.acrt_lname,
    mc.role
FROM movie m
JOIN movie_cast mc
    ON m.mov_id = mc.mov_id
JOIN actor a
    ON mc.act_id = a.act_id
WHERE m.mov_title = 'Annie Hall';
```
#### Output
| act_fname | acrt_lname | role        |
| :-------- | :--------- | :---------- |
| Woody     | Allen      | Alvy Singer |

## Question 29
***Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.***
#### SQL Query
```sql
SELECT dr.dir_fname, dr.dir_lname, mv.mov_title FROM movie mv 
JOIN movie_direction md 
    ON mv.mov_id = md mov_id 
JOIN director dr 
    ON md.dir_id =dr.dir_id 
WHERE mv.mov_title = 'Eyes Wide Shut';
```
#### Output
| dir_fname | dir_lname | mov_title      |
| :-------- | :-------- | :------------- |
| Stanley   | Kubrick   | Eyes Wide Shut |

## Question 30
***Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.***
#### SQL Query
```sql
select dr.dir_fname, dr.dir_lname, mv.mov_title from movie_direction md join movie mv 
on md.mov_id = mv.mov_id join director dr on md.dir_id = dr.dir_id join movie_cast mc 
on mv.mov_id = mc.mov_id where mc.role = 'Sean Maguire';
```
#### Output
| dir_fname | dir_lname | mov_title         |
| :-------- | :-------- | :---------------- |
| Gus       | Van Sant  | Good Will Hunting |

## Question 31
***Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.***
#### SQL Query
````sql
SELECT 
actor.act_fname AS actor_firstname,
actor.acrt_lname AS actor_lastname,
movie.mov_title AS movie_title,
movie.mov_year AS release_year
FROM actor 
JOIN movie_cast
ON actor.act_id = movie_cast.act_id
JOIN movie
ON movie.mov_id = movie_cast.mov_id
WHERE movie.mov_year NOT BETWEEN 1990 AND 2000;
````
#### Output
| actor_firstname | actor_lastname | movie_title         | release_year |
| :-------------- | :------------- | :------------------ | :----------- |
| James           | Stewart        | Vertigo             | 1958         |
| Deborah         | Kerr           | The Innocents       | 1961         |
| Peter           | OToole         | Lawrence of Arabia  | 1962         |
| Robert          | De Niro        | The Deer Hunter     | 1978         |
| F. Murray       | Abraham        | Amadeus             | 1984         |
| Harrison        | Ford           | Blade Runner        | 1982         |
| Jack            | Nicholson      | Chinatown           | 1974         |
| Woody           | Allen          | Annie Hall          | 1977         |
| Kevin           | Spacey         | Beyond the Sea      | 2004         |
| Jon             | Voight         | Deliverance         | 1972         |
| Christian       | Bale           | Chinatown           | 1974         |
| Maggie          | Gyllenhaal     | Donnie Darko        | 2001         |
| Dev             | Patel          | Slumdog Millionaire | 2008         |
| Sigourney       | Weaver         | Aliens              | 1986         |

## Question 32
***Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.***
#### SQL Query
````sql
SELECT
director.dir_fname AS director_firstName,
director.dir_lname AS director_lastName,
COUNT(DISTINCT genres.gen_id) AS number_of_genres
FROM director
JOIN movie_direction
ON director.dir_id = movie_direction.dir_id
JOIN movie_genres
ON movie_direction.mov_id = movie_genres.mov_id
JOIN genres
ON movie_genres.gen_id = genres.gen_id
GROUP BY director.dir_fname, director.dir_lname
HAVING COUNT(DISTINCT genres.gen_title) > 1
ORDER BY director.dir_fname, director.dir_lname;
````
#### Output
| director_firstName | director_lastName | number_of_genres |
| :----------------- | :---------------- | :--------------- |

## Question 33
***Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    m.mov_year,
    g.gen_title
FROM movie m
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
ORDER BY m.mov_title, g.gen_title;
```
#### Output
| mov_title                | mov_year | gen_title |
| :----------------------- | :------- | :-------- |
| Aliens                   | 1986     | Action    |
| American Beauty          | 1999     | Romance   |
| Annie Hall               | 1977     | Comedy    |
| Back to the Future       | 1985     | Mystery   |
| Beyond the Sea           | 2004     | Music     |
| Blade Runner             | 1982     | Thriller  |
| Braveheart               | 1995     | Drama     |
| Deliverance              | 1972     | Adventure |
| Eyes Wide Shut           | 1999     | Mystery   |
| Lawrence of Arabia       | 1962     | Adventure |
| Princess Mononoke        | 1997     | Animation |
| Slumdog Millionaire      | 2008     | Drama     |
| Spirited Away            | 2001     | Drama     |
| The Deer Hunter          | 1978     | War       |
| The Innocents            | 1961     | Horror    |
| The Shawshank Redemption | 1994     | Crime     |
| The Usual Suspects       | 1995     | Crime     |
| Trainspotting            | 1996     | Drama     |
| Vertigo                  | 1958     | Mystery   |

## Question 34
***Write a SQL query to find all the movies with year, genres, and name of the director.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    m.mov_year,
    g.gen_title,
    d.dir_fname,
    d.dir_lname
FROM movie m
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
ORDER BY m.mov_title, g.gen_title;
```
#### Output
| movie_title              | movie_year | genre_title | director_first_name | director_last_name |
| :----------------------- | :--------- | :---------- | :------------------ | :----------------- |
| Aliens                   | 1986       | Action      | James               | Cameron            |
| American Beauty          | 1999       | Romance     | Sam                 | Mendes             |
| Annie Hall               | 1977       | Comedy      | Woody               | Allen              |
| Beyond the Sea           | 2004       | Music       | Kevin               | Spacey             |
| Blade Runner             | 1982       | Thriller    | Ridley              | Scott              |
| Deliverance              | 1972       | Adventure   | John                | Boorman            |
| Eyes Wide Shut           | 1999       | Mystery     | Stanley             | Kubrick            |
| Lawrence of Arabia       | 1962       | Adventure   | David               | Lean               |
| Princess Mononoke        | 1997       | Animation   | Hayao               | Miyazaki           |
| Slumdog Millionaire      | 2008       | Drama       | Danny               | Boyle              |
| The Deer Hunter          | 1978       | War         | Michael             | Cimino             |
| The Innocents            | 1961       | Horror      | Jack                | Clayton            |
| The Shawshank Redemption | 1994       | Crime       | Frank               | Darabont           |
| The Usual Suspects       | 1995       | Crime       | Bryan               | Singer             |
| Trainspotting            | 1996       | Drama       | Danny               | Boyle              |
| Vertigo                  | 1958       | Mystery     | Alfred              | Hitchcock          |

## Question 35
***Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    m.mov_year,
    m.mov_dt_rel,
    m.mov_time,
    d.dir_fname,
    d.dir_lname
FROM movie m
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
WHERE m.mov_dt_rel < '1989-01-01'
    AND m.mov_dt_rel IS NOT NULL
    AND m.mov_dt_rel != ''
ORDER BY m.mov_dt_rel DESC;
```
#### Output
| movie_title        | release_year | date_of_release | duration | director_first_name | director_last_name |
| :----------------- | :----------- | :-------------- | :------- | :------------------ | :----------------- |
| Aliens             | 1986         | 1986-08-29      | 137      | James               | Cameron            |
| Amadeus            | 1984         | 1985-01-07      | 160      | Milos               | Forman             |
| Deliverance        | 1972         | 1982-10-05      | 109      | John                | Boorman            |
| Blade Runner       | 1982         | 1982-09-09      | 117      | Ridley              | Scott              |
| The Deer Hunter    | 1978         | 1979-03-08      | 183      | Michael             | Cimino             |
| Annie Hall         | 1977         | 1977-04-20      | 93       | Woody               | Allen              |
| Chinatown          | 1974         | 1974-08-09      | 130      | Roman               | Polanski           |
| Lawrence of Arabia | 1962         | 1962-12-11      | 216      | David               | Lean               |
| The Innocents      | 1961         | 1962-02-19      | 100      | Jack                | Clayton            |
| Vertigo            | 1958         | 1958-08-24      | 128      | Alfred              | Hitchcock          |
|                    |              |                 |          |                     |                    |

## Question 36
***Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.***
#### SQL Query
```sql
SELECT 
    g.gen_title,
    AVG(m.mov_time),
    COUNT(m.mov_id)
FROM genres g
INNER JOIN movie_genres mg ON g.gen_id = mg.gen_id
INNER JOIN movie m ON mg.mov_id = m.mov_id
WHERE m.mov_time IS NOT NULL
GROUP BY g.gen_title
ORDER BY g.gen_title;
```
#### Output
| gen_title | (No column name) | (No column name) |
| :-------- | :--------------- | :--------------- |
| Action    | 137              | 1                |
| Adventure | 162              | 2                |
| Animation | 134              | 1                |
| Comedy    | 93               | 1                |
| Crime     | 124              | 2                |
| Drama     | 129              | 4                |
| Horror    | 100              | 1                |
| Music     | 118              | 1                |
| Mystery   | 134              | 3                |
| Romance   | 122              | 1                |
| Thriller  | 117              | 1                |
| War       | 183              | 1                |
|           |                  |                  |

## Question 37
***Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    m.mov_year,
    d.dir_fname,
    d.dir_lname,
    a.act_fname,
    a.acrt_lname,
    mc.role AS role
FROM movie m
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
INNER JOIN movie_cast mc ON m.mov_id = mc.mov_id
INNER JOIN actor a ON mc.act_id = a.act_id
WHERE m.mov_time = (
    SELECT MIN(mov_time)
    FROM movie
    WHERE mov_time IS NOT NULL
)
ORDER BY m.mov_title, a.act_fname;
```
#### Output
| mov_title  | mov_year | dir_fname | dir_lname | act_fname | acrt_lname | role        |
| :--------- | :------- | :-------- | :-------- | :-------- | :--------- | :---------- |
| Annie Hall | 1977     | Woody     | Allen     | Woody     | Allen      | Alvy Singer |

## Question 38
***Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.***
#### SQL Query
```sql
SELECT DISTINCT m.mov_year
FROM movie m
INNER JOIN movie_rating rt ON m.mov_id = rt.mov_id
WHERE rt.rev_stars IN (3, 4)
ORDER BY m.mov_year ASC;
```
#### Output
| mov_year |
| :------- |
| 1997     |

## Question 39
***Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.***
#### SQL Query
```sql
SELECT 
    mr.rev_name,
    m.mov_title ,
    rt.rev_stars
FROM movie_rating rt
INNER JOIN movie_reviewer mr ON rt.rev_id = mr.rev_id
INNER JOIN movie m ON rt.mov_id = m.mov_id
WHERE mr.rev_name != ''
ORDER BY mr.rev_name ASC, m.mov_title ASC, rt.rev_stars ASC;
```
#### Output
| reviewer_name      | movie_title         | review_stars |
| :----------------- | :------------------ | :----------- |
| Brandt Sponseller  | Aliens              | 8.40         |
| Flagrant Baronessa | Lawrence of Arabia  | 8.30         |
| Hannah Steele      | Donnie Darko        | 8.10         |
| Jack Malvern       | The Innocents       | 7.90         |
| Josh Cates         | Good Will Hunting   | 4.00         |
| Krug Stillo        | Seven Samurai       | 7.70         |
| Mike Salvati       | Annie Hall          | 8.10         |
| Neal Wruck         | Chinatown           | NULL         |
| Paul Monks         | Boogie Nights       | 3.00         |
| Richard Adams      | Beyond the Sea      | 6.70         |
| Righty Sock        | Titanic             | 7.70         |
| Righty Sock        | Vertigo             | 8.40         |
| Sasha Goldshtein   | American Beauty     | 7.00         |
| Scott LeBrun       | Trainspotting       | NULL         |
| Simon Wright       | The Usual Suspects  | 8.60         |
| Victor Woeltjen    | Avatar              | 7.30         |
| Vincent Cadena     | Slumdog Millionaire | 8.00         |

## Question 40
***Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    MAX(rt.rev_stars) AS max_review
FROM movie m
INNER JOIN movie_rating rt ON m.mov_id = rt.mov_id
WHERE rt.rev_stars IS NOT NULL
GROUP BY m.mov_title
ORDER BY m.mov_title ASC;
```
#### Output
| reviewer_name      | movie_title         | review_stars |
| :----------------- | :------------------ | :----------- |
| Brandt Sponseller  | Aliens              | 8.40         |
| Flagrant Baronessa | Lawrence of Arabia  | 8.30         |
| Hannah Steele      | Donnie Darko        | 8.10         |
| Jack Malvern       | The Innocents       | 7.90         |
| Josh Cates         | Good Will Hunting   | 4.00         |
| Krug Stillo        | Seven Samurai       | 7.70         |
| Mike Salvati       | Annie Hall          | 8.10         |
| Neal Wruck         | Chinatown           | NULL         |
| Paul Monks         | Boogie Nights       | 3.00         |
| Richard Adams      | Beyond the Sea      | 6.70         |
| Righty Sock        | Titanic             | 7.70         |
| Righty Sock        | Vertigo             | 8.40         |
| Sasha Goldshtein   | American Beauty     | 7.00         |
| Scott LeBrun       | Trainspotting       | NULL         |
| Simon Wright       | The Usual Suspects  | 8.60         |
| Victor Woeltjen    | Avatar              | 7.30         |
| Vincent Cadena     | Slumdog Millionaire | 8.00         |

## Question 41
***Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.***
#### SQL Query
```sql
SELECT DISTINCT
    m.mov_title,
    d.dir_fname,
    d.dir_lname,
    rt.rev_stars
FROM movie m
INNER JOIN movie_rating rt ON m.mov_id = rt.mov_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
WHERE rt.rev_stars IS NOT NULL
ORDER BY m.mov_title, rt.rev_stars;
```
#### Output
| mov_title           | dir_fname | dir_lname       | rev_stars |
| :------------------ | :-------- | :-------------- | :-------- |
| Aliens              | James     | Cameron         | 8.40      |
| American Beauty     | Sam       | Mendes          | 7.00      |
| Annie Hall          | Woody     | Allen           | 8.10      |
| Beyond the Sea      | Kevin     | Spacey          | 6.70      |
| Blade Runner        | Ridley    | Scott           | 8.20      |
| Boogie Nights       | Paul      | Thomas Anderson | 3.00      |
| Donnie Darko        | Richard   | Kelly           | 8.10      |
| Good Will Hunting   | Gus       | Van Sant        | 4.00      |
| Lawrence of Arabia  | David     | Lean            | 8.30      |
| Princess Mononoke   | Hayao     | Miyazaki        | 8.40      |
| Slumdog Millionaire | Danny     | Boyle           | 8.00      |
| The Innocents       | Jack      | Clayton         | 7.90      |
| The Usual Suspects  | Bryan     | Singer          | 8.60      |
| Titanic             | James     | Cameron         | 7.70      |
| Vertigo             | Alfred    | Hitchcock       | 8.40      |

## Question 42
***Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    a.act_fname,
    a.acrt_lname,
    mc.role
FROM movie m
INNER JOIN movie_cast mc ON m.mov_id = mc.mov_id
INNER JOIN actor a ON mc.act_id = a.act_id
WHERE a.act_id IN (
    SELECT act_id
    FROM movie_cast
    GROUP BY act_id
    HAVING COUNT(DISTINCT mov_id) > 1
)
ORDER BY m.mov_title, a.act_fname;
```
#### Output
| mov_title       | act_fname | acrt_lname | role           |
| :-------------- | :-------- | :--------- | :------------- |
| American Beauty | Kevin     | Spacey     | Lester Burnham |
| Beyond the Sea  | Kevin     | Spacey     | Bobby Darin    |

## Question 43
***Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.***
#### SQL Query
```sql
SELECT
director.dir_fname AS director_firstName,
director.dir_lname AS director_lastName,
movie.mov_title AS movie_title,
actor.act_fname AS actor_first_name,
actor.acrt_lname AS actor_last_name,
movie_cast.role AS role
FROM actor
JOIN movie_cast
ON actor.act_id = movie_cast.act_id
JOIN movie
ON movie_cast.mov_id = movie.mov_id
JOIN movie_direction
ON movie.mov_id = movie_direction.mov_id
JOIN director
ON movie_direction.dir_id = director.dir_id
WHERE
actor.act_fname = 'Claire' AND actor.acrt_lname = 'Danes';
```
#### Output
| director_firstName | director_lastName | movie_title       | actor_first_name | actor_last_name | role |
| :----------------- | :---------------- | :---------------- | :--------------- | :-------------- | :--- |
| Hayao              | Miyazaki          | Princess Mononoke | Claire           | Danes           | San  |

## Question 44
***Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.***
#### SQL Query
```sql
SELECT 
    a.act_fname,
    a.acrt_lname,
    m.mov_title,
    mc.role
FROM actor a
INNER JOIN movie_cast mc ON a.act_id = mc.act_id
INNER JOIN movie m ON mc.mov_id = m.mov_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
WHERE a.act_fname = d.dir_fname 
    AND a.acrt_lname = d.dir_lname
ORDER BY a.act_fname, m.mov_title;
```
#### Output
| actor_first_name | actor_last_name | movie_title    | role        |
| :--------------- | :-------------- | :------------- | :---------- |
| Kevin            | Spacey          | Beyond the Sea | Bobby Darin |
| Woody            | Allen           | Annie Hall     | Alvy Singer |

## Question 45
***Write a SQL query to find the cast list of the movie ‘Chinatown’. Return first name, last name.***
#### SQL Query
```sql
SELECT 
    a.act_fname,
    a.acrt_lname
FROM actor a
INNER JOIN movie_cast mc ON a.act_id = mc.act_id
INNER JOIN movie m ON mc.mov_id = m.mov_id
WHERE m.mov_title = 'Chinatown'
ORDER BY a.act_fname, a.acrt_lname;
```
#### Output
| first_name | last_name |
| :--------- | :-------- |
| Christian  | Bale      |
| Jack       | Nicholson |

## Question 46
***Write a SQL query to find those movies where actor’s first name is 'Harrison' and last name is 'Ford'. Return movie title.***
#### SQL Query
```sql
SELECT DISTINCT
    m.mov_title AS movie_title
FROM movie m
INNER JOIN movie_cast mc ON m.mov_id = mc.mov_id
INNER JOIN actor a ON mc.act_id = a.act_id
WHERE a.act_fname = 'Harrison' 
    AND a.acrt_lname = 'Ford'
ORDER BY m.mov_title;
```
#### Output
| movie_title  |
| :----------- |
| Blade Runner |

## Question 47
***Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    m.mov_year,
    rt.rev_stars,
    m.mov_rel_country
FROM movie m
INNER JOIN movie_rating rt ON m.mov_id = rt.mov_id
WHERE rt.rev_stars = (
    SELECT MAX(rev_stars)
    FROM movie_rating
    WHERE rev_stars IS NOT NULL
)
ORDER BY m.mov_title;
```
#### Output
| movie_title        | movie_year | review_stars | releasing_country |
| :----------------- | :--------- | :----------- | :---------------- |
| The Usual Suspects | 1995       | 8.60         | UK                |

## Question 48
***Write a SQL query to find the highest-rated ‘Mystery Movies’. Return the title, year, and rating.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    m.mov_year,
    rt.rev_stars
FROM movie m
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
INNER JOIN movie_rating rt ON m.mov_id = rt.mov_id
WHERE g.gen_title = 'Mystery'
    AND rt.rev_stars = (
        SELECT MAX(rt2.rev_stars)
        FROM movie_rating rt2
        INNER JOIN movie_genres mg2 ON rt2.mov_id = mg2.mov_id
        INNER JOIN genres g2 ON mg2.gen_id = g2.gen_id
        WHERE g2.gen_title = 'Mystery'
            AND rt2.rev_stars IS NOT NULL
    )
ORDER BY m.mov_title;
```
#### Output
| movie_title | movie_year | rating |
| :---------- | :--------- | :----- |
| Vertigo     | 1958       | 8.40   |

## Question 49
***Write a SQL query to find the years when most of the ‘Mystery Movies’ produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.***
#### SQL Query
```sql
SELECT 
    m.mov_year,
    g.gen_title,
    COUNT(g.gen_title),
    AVG(rt.rev_stars)
FROM movie m
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
INNER JOIN movie_rating rt ON m.mov_id = rt.mov_id
WHERE g.gen_title = 'Mystery'
    AND rt.rev_stars IS NOT NULL
GROUP BY m.mov_year, g.gen_title
ORDER BY m.mov_year ASC;
```
#### Output
| movie_year | genre_title | number_of_movies | average_rating |
| :--------- | :---------- | :--------------- | :------------- |
| 1958       | Mystery     | 1                | 8.400000       |

## Question 50
***Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.***
#### SQL Query
```sql
SELECT 
    m.mov_title,
    a.act_fname,
    a.acrt_lname,
    m.mov_year,
    mc.role,
    g.gen_title,
    d.dir_fname,
    d.dir_lname,
    m.mov_dt_rel,
    rt.rev_stars
FROM actor a
INNER JOIN movie_cast mc ON a.act_id = mc.act_id
INNER JOIN movie m ON mc.mov_id = m.mov_id
INNER JOIN movie_genres mg ON m.mov_id = mg.mov_id
INNER JOIN genres g ON mg.gen_id = g.gen_id
INNER JOIN movie_direction md ON m.mov_id = md.mov_id
INNER JOIN director d ON md.dir_id = d.dir_id
LEFT JOIN movie_rating rt ON m.mov_id = rt.mov_id
WHERE a.act_gender = 'F'
ORDER BY m.mov_title, a.act_fname, g.gen_title;
```
#### Output
| movie_title       | actor_first_name | actor_last_name | movie_year | role          | genre_title | director_first_name | director_last_name | date_of_release | rating |
| :---------------- | :--------------- | :-------------- | :--------- | :------------ | :---------- | :------------------ | :----------------- | :-------------- | :----- |
| Aliens            | Sigourney        | Weaver          | 1986       | Ripley        | Action      | James               | Cameron            | 1986-08-29      | 8.40   |
| Eyes Wide Shut    | Nicole           | Kidman          | 1999       | Alice Harford | Mystery     | Stanley             | Kubrick            | 1900-01-01      | NULL   |
| Princess Mononoke | Claire           | Danes           | 1997       | San           | Animation   | Hayao               | Miyazaki           | 2001-10-19      | 8.40   |
| The Innocents     | Deborah          | Kerr            | 1961       | Miss Giddens  | Horror      | Jack                | Clayton            | 1962-02-19      | 7.90   |
