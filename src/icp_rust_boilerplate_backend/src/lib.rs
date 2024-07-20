#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Book {
    id: u64,
    title: String,
    author: String,
    genre: String,
    publication_year: i32,
    isbn: String,
    location: String,
    available: bool,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Member {
    id: u64,
    username: String,
    phone_number: String,
    address: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Loan {
    id: u64,
    book_id: u64,
    member_id: u64,
    loan_date: u64,
    due_date: u64,
    return_date: Option<u64>,
    fine: f64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Reservation {
    id: u64,
    book_id: u64,
    member_id: u64,
    reservation_date: u64,
}

impl Storable for Book {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Book {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Member {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Member {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Loan {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Loan {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Reservation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Reservation {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static BOOK_STORAGE: RefCell<StableBTreeMap<u64, Book, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static MEMBER_STORAGE: RefCell<StableBTreeMap<u64, Member, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static LOAN_STORAGE: RefCell<StableBTreeMap<u64, Loan, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static RESERVATION_STORAGE: RefCell<StableBTreeMap<u64, Reservation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct BookPayload {
    title: String,
    author: String,
    genre: String,
    publication_year: i32,
    isbn: String,
    location: String,
    available: bool,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct MemberPayload {
    username: String,
    phone_number: String,
    address: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct LoanPayload {
    book_id: u64,
    member_id: u64,
    due_date: u64,
    return_date: Option<u64>,
    fine: f64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ReservationPayload {
    book_id: u64,
    member_id: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
}

#[ic_cdk::update]
fn create_book(payload: BookPayload) -> Result<Book, Message> {
    if payload.title.is_empty() || payload.author.is_empty() || payload.isbn.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'title', 'author', and 'isbn' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let book = Book {
        id,
        title: payload.title,
        author: payload.author,
        genre: payload.genre,
        publication_year: payload.publication_year,
        isbn: payload.isbn,
        location: payload.location,
        available: payload.available,
        created_at: current_time(),
    };
    BOOK_STORAGE.with(|storage| storage.borrow_mut().insert(id, book.clone()));
    Ok(book)
}

#[ic_cdk::query]
fn get_books() -> Result<Vec<Book>, Message> {
    BOOK_STORAGE.with(|storage| {
        let books: Vec<Book> = storage
            .borrow()
            .iter()
            .map(|(_, book)| book.clone())
            .collect();

        if books.is_empty() {
            Err(Message::NotFound("No books found".to_string()))
        } else {
            Ok(books)
        }
    })
}

#[ic_cdk::query]
fn get_book_by_id(id: u64) -> Result<Book, Message> {
    BOOK_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, book)| book.id == id)
            .map(|(_, book)| book.clone())
            .ok_or(Message::NotFound("Book not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_book(id: u64, payload: BookPayload) -> Result<Book, Message> {
    if payload.title.is_empty() || payload.author.is_empty() || payload.isbn.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'title', 'author', and 'isbn' are provided.".to_string(),
        ));
    }

    BOOK_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        match storage.iter().find(|(_, book)| book.id == id).map(|(_, book)| book.clone()) {
            Some(mut book) => {
                book.title = payload.title;
                book.author = payload.author;
                book.genre = payload.genre;
                book.publication_year = payload.publication_year;
                book.isbn = payload.isbn;
                book.location = payload.location;
                book.available = payload.available;
                storage.insert(id, book.clone());
                Ok(book)
            }
            None => Err(Message::NotFound("Book not found".to_string())),
        }
    })
}

#[ic_cdk::update]
fn delete_book(id: u64) -> Result<Message, Message> {
    BOOK_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(Message::Success("Book deleted successfully".to_string()))
        } else {
            Err(Message::NotFound("Book not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn create_member(payload: MemberPayload) -> Result<Member, Message> {
    if payload.username.is_empty() || payload.phone_number.is_empty() || payload.address.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'username', 'phone_number', and 'address' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let member = Member {
        id,
        username: payload.username,
        phone_number: payload.phone_number,
        address: payload.address,
        created_at: current_time(),
    };
    MEMBER_STORAGE.with(|storage| storage.borrow_mut().insert(id, member.clone()));
    Ok(member)
}

#[ic_cdk::query]
fn get_members() -> Result<Vec<Member>, Message> {
    MEMBER_STORAGE.with(|storage| {
        let members: Vec<Member> = storage
            .borrow()
            .iter()
            .map(|(_, member)| member.clone())
            .collect();

        if members.is_empty() {
            Err(Message::NotFound("No members found".to_string()))
        } else {
            Ok(members)
        }
    })
}

#[ic_cdk::query]
fn get_member_by_id(id: u64) -> Result<Member, Message> {
    MEMBER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, member)| member.id == id)
            .map(|(_, member)| member.clone())
            .ok_or(Message::NotFound("Member not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_member(id: u64, payload: MemberPayload) -> Result<Member, Message> {
    if payload.username.is_empty() || payload.phone_number.is_empty() || payload.address.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'username', 'phone_number', and 'address' are provided.".to_string(),
        ));
    }

    MEMBER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        match storage.iter().find(|(_, member)| member.id == id).map(|(_, member)| member.clone()) {
            Some(mut member) => {
                member.username = payload.username;
                member.phone_number = payload.phone_number;
                member.address = payload.address;
                storage.insert(id, member.clone());
                Ok(member)
            }
            None => Err(Message::NotFound("Member not found".to_string())),
        }
    })
}

#[ic_cdk::update]
fn delete_member(id: u64) -> Result<Message, Message> {
    MEMBER_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(Message::Success("Member deleted successfully".to_string()))
        } else {
            Err(Message::NotFound("Member not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn create_loan(payload: LoanPayload) -> Result<Loan, Message> {
    if payload.due_date == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'due_date' is provided.".to_string(),
        ));
    }

    let book = BOOK_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, book)| book.id == payload.book_id)
            .map(|(_, book)| book.clone())
    });
    if book.is_none() {
        return Err(Message::NotFound("Book not found".to_string()));
    }

    let member = MEMBER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, member)| member.id == payload.member_id)
            .map(|(_, member)| member.clone())
    });
    if member.is_none() {
        return Err(Message::NotFound("Member not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let loan = Loan {
        id,
        book_id: payload.book_id,
        member_id: payload.member_id,
        loan_date: current_time(),
        due_date: payload.due_date,
        return_date: payload.return_date,
        fine: payload.fine,
    };
    LOAN_STORAGE.with(|storage| storage.borrow_mut().insert(id, loan.clone()));
    Ok(loan)
}

#[ic_cdk::query]
fn get_book_loans() -> Result<Vec<Loan>, Message> {
    LOAN_STORAGE.with(|storage| {
        let loans: Vec<Loan> = storage
            .borrow()
            .iter()
            .map(|(_, loan)| loan.clone())
            .collect();

        if loans.is_empty() {
            Err(Message::NotFound("No loans found".to_string()))
        } else {
            Ok(loans)
        }
    })
}

#[ic_cdk::query]
fn get_book_loan_by_id(id: u64) -> Result<Loan, Message> {
    LOAN_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, loan)| loan.id == id)
            .map(|(_, loan)| loan.clone())
            .ok_or(Message::NotFound("Loan not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_loan(id: u64, payload: LoanPayload) -> Result<Loan, Message> {
    if payload.due_date == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'due_date' is provided.".to_string(),
        ));
    }

    LOAN_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        match storage.iter().find(|(_, loan)| loan.id == id).map(|(_, loan)| loan.clone()) {
            Some(mut loan) => {
                loan.book_id = payload.book_id;
                loan.member_id = payload.member_id;
                loan.due_date = payload.due_date;
                loan.return_date = payload.return_date;
                loan.fine = payload.fine;
                storage.insert(id, loan.clone());
                Ok(loan)
            }
            None => Err(Message::NotFound("Loan not found".to_string())),
        }
    })
}

#[ic_cdk::update]
fn delete_loan(id: u64) -> Result<Message, Message> {
    LOAN_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(Message::Success("Loan deleted successfully".to_string()))
        } else {
            Err(Message::NotFound("Loan not found".to_string()))
        }
    })
}

#[ic_cdk::update]
fn create_reservation(payload: ReservationPayload) -> Result<Reservation, Message> {
    if payload.book_id == 0 || payload.member_id == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'book_id' and 'member_id' are provided.".to_string(),
        ));
    }

    let book = BOOK_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, book)| book.id == payload.book_id)
            .map(|(_, book)| book.clone())
    });
    if book.is_none() {
        return Err(Message::NotFound("Book not found".to_string()));
    }

    let member = MEMBER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, member)| member.id == payload.member_id)
            .map(|(_, member)| member.clone())
    });
    if member.is_none() {
        return Err(Message::NotFound("Member not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let reservation = Reservation {
        id,
        book_id: payload.book_id,
        member_id: payload.member_id,
        reservation_date: current_time(),
    };
    RESERVATION_STORAGE.with(|storage| storage.borrow_mut().insert(id, reservation.clone()));
    Ok(reservation)
}

#[ic_cdk::query]
fn get_reservations() -> Result<Vec<Reservation>, Message> {
    RESERVATION_STORAGE.with(|storage| {
        let reservations: Vec<Reservation> = storage
            .borrow()
            .iter()
            .map(|(_, reservation)| reservation.clone())
            .collect();

        if reservations.is_empty() {
            Err(Message::NotFound("No reservations found".to_string()))
        } else {
            Ok(reservations)
        }
    })
}

#[ic_cdk::query]
fn get_reservation_by_id(id: u64) -> Result<Reservation, Message> {
    RESERVATION_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, reservation)| reservation.id == id)
            .map(|(_, reservation)| reservation.clone())
            .ok_or(Message::NotFound("Reservation not found".to_string()))
    })
}

#[ic_cdk::update]
fn update_reservation(id: u64, payload: ReservationPayload) -> Result<Reservation, Message> {
    if payload.book_id == 0 || payload.member_id == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'book_id' and 'member_id' are provided.".to_string(),
        ));
    }

    RESERVATION_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        match storage.iter().find(|(_, reservation)| reservation.id == id).map(|(_, reservation)| reservation.clone()) {
            Some(mut reservation) => {
                reservation.book_id = payload.book_id;
                reservation.member_id = payload.member_id;
                storage.insert(id, reservation.clone());
                Ok(reservation)
            }
            None => Err(Message::NotFound("Reservation not found".to_string())),
        }
    })
}

#[ic_cdk::update]
fn delete_reservation(id: u64) -> Result<Message, Message> {
    RESERVATION_STORAGE.with(|storage| {
        let mut storage = storage.borrow_mut();
        if storage.remove(&id).is_some() {
            Ok(Message::Success("Reservation deleted successfully".to_string()))
        } else {
            Err(Message::NotFound("Reservation not found".to_string()))
        }
    })
}

fn current_time() -> u64 {
    time()
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

ic_cdk::export_candid!();
