create table books {
    isbn varchar not null,
    title varchar not null,
    author varchar not null,
    summary varchar not null
};

create unique index books_isbn_idx on books (isbn);