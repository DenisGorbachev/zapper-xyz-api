use errgonomic::handle_opt;
use page_turner::TurnedPage;
use thiserror::Error;

pub trait CursorPage {
    fn has_next_page(&self) -> bool;

    fn end_cursor(&self) -> Option<&str>;
}

pub trait CursorPaginatedRequest {
    fn cursor_after(&self) -> Option<&str>;

    fn set_cursor_after(&mut self, after: String);
}

pub fn turn_cursor_page<Request, Page>(mut request: Request, page: Page) -> Result<TurnedPage<Page, Request>, TurnCursorPageError<Request>>
where
    Request: CursorPaginatedRequest,
    Page: CursorPage,
{
    use TurnCursorPageError::*;
    if page.has_next_page() {
        let after = handle_opt!(page.end_cursor(), EndCursorNotFound, request);
        if request.cursor_after() == Some(after) {
            Ok(TurnedPage::last(page))
        } else {
            request.set_cursor_after(after.to_owned());
            Ok(TurnedPage::next(page, request))
        }
    } else {
        Ok(TurnedPage::last(page))
    }
}

#[derive(Error, Debug)]
pub enum TurnCursorPageError<Request> {
    #[error("cursor page info did not contain an end cursor")]
    EndCursorNotFound { request: Request },
}
