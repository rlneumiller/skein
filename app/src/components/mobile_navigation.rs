use crate::components::navigation::Navigation;
use leptos::{logging::log, prelude::*};

#[component]
fn MenuIcon(class: String) -> impl IntoView {
    view! {
      <svg
        aria-hidden="true"
        viewBox="0 0 24 24"
        fill="none"
        strokeWidth="2"
        strokeLinecap="round"
        class=class
      >
        <path d="M4 7h16M4 12h16M4 17h16" />
      </svg>
    }
}

#[component]
fn CloseIcon(class: String) -> impl IntoView {
    view! {
      <svg
        aria-hidden="true"
        viewBox="0 0 24 24"
        fill="none"
        strokeWidth="2"
        strokeLinecap="round"
        class=class
      >
        <path d="M5 5l14 14M19 5l-14 14" />
      </svg>
    }
}

#[component]
pub fn MobileNavigation() -> impl IntoView {
    view! {
        <DialogMenu>
            <Navigation
              class="mt-5 px-1".to_string()
            />
        </DialogMenu>
    }
}

#[component]
fn Logo(class: String) -> impl IntoView {
    view! {
        <svg
        class=class
        xmlns="http://www.w3.org/2000/svg"
        viewBox="0 0 277.97 82.94"><path d="M31.13 82.94c-6.67 0-12.48-.97-17.44-2.92S4.17 74.98 0 70.73l13.75-13.75c2.86 2.71 5.87 4.79 9.02 6.22 3.15 1.43 6.53 2.14 10.12 2.14 3.01 0 5.28-.46 6.82-1.38 1.54-.92 2.31-2.18 2.31-3.79s-.66-2.95-1.98-4.02c-1.32-1.06-3.06-2.02-5.23-2.86-2.16-.84-4.55-1.7-7.15-2.58-2.6-.88-5.19-1.94-7.76-3.19a35.168 35.168 0 0 1-7.09-4.56c-2.16-1.8-3.91-4.03-5.23-6.71-1.32-2.68-1.98-5.96-1.98-9.85 0-5.06 1.21-9.42 3.63-13.09s5.83-6.47 10.23-8.42c4.4-1.94 9.57-2.92 15.51-2.92s11.31.94 16.33 2.81c5.02 1.87 9.19 4.53 12.48 7.98L49.92 26.51c-2.42-2.35-4.84-4.09-7.26-5.23-2.42-1.14-5.06-1.71-7.92-1.71-2.27 0-4.09.37-5.44 1.1-1.36.73-2.04 1.83-2.04 3.3 0 1.54.66 2.81 1.98 3.79 1.32.99 3.06 1.89 5.23 2.7 2.16.81 4.55 1.65 7.15 2.53 2.6.88 5.19 1.93 7.75 3.14 2.57 1.21 4.93 2.75 7.1 4.62 2.16 1.87 3.9 4.2 5.23 6.99 1.32 2.79 1.98 6.16 1.98 10.12 0 7.99-2.84 14.17-8.52 18.54-5.68 4.36-13.7 6.54-24.04 6.54ZM71.5 81.62V1.1h21.56v80.52H71.5Zm34.87 0L91.3 53.13l14.96-25.63h23.21l-19.36 28.93.55-7.04 19.91 32.23h-24.2Z"/><path d="M162.03 82.94c-6.23 0-11.77-1.23-16.61-3.69s-8.65-5.83-11.44-10.12c-2.79-4.29-4.18-9.19-4.18-14.69s1.32-10.27 3.96-14.52c2.64-4.25 6.27-7.61 10.89-10.06 4.62-2.46 9.86-3.69 15.73-3.69 6.38 0 11.79 1.48 16.22 4.46 4.44 2.97 7.68 7.28 9.74 12.92l-34.21 32.01-10.12-10.45 30.91-28.82-2.2 13.42c-.66-2.57-1.87-4.51-3.63-5.83-1.76-1.32-4-1.98-6.71-1.98-2.35 0-4.36.46-6.05 1.38-1.69.92-2.97 2.24-3.85 3.96-.88 1.72-1.32 3.79-1.32 6.21 0 2.86.61 5.39 1.82 7.59s2.9 3.92 5.06 5.17c2.16 1.25 4.6 1.87 7.32 1.87 2.27 0 4.31-.42 6.1-1.27 1.8-.84 3.61-2.25 5.45-4.23l10.78 10.67c-3.15 3.3-6.67 5.74-10.56 7.31-3.89 1.58-8.25 2.37-13.09 2.37ZM203.06 22.44c-3.15 0-5.77-1.08-7.87-3.24-2.09-2.16-3.13-4.82-3.13-7.98s1.04-5.81 3.13-7.98c2.09-2.16 4.71-3.25 7.87-3.25s5.87 1.08 7.92 3.25c2.05 2.16 3.08 4.82 3.08 7.98s-1.03 5.81-3.08 7.98c-2.05 2.16-4.69 3.24-7.92 3.24Zm-10.78 59.18V27.5h21.56v54.12h-21.56ZM222.64 81.62V51.7c0-5.13 1.17-9.61 3.52-13.42 2.35-3.81 5.61-6.78 9.79-8.91 4.18-2.13 8.98-3.19 14.41-3.19s10.32 1.06 14.46 3.19c4.14 2.13 7.37 5.1 9.68 8.91 2.31 3.81 3.46 8.29 3.46 13.42v29.92H256.4V51.48c0-2.27-.55-4.01-1.65-5.23-1.1-1.21-2.57-1.81-4.4-1.81s-3.41.6-4.51 1.81c-1.1 1.21-1.65 2.95-1.65 5.23v30.14h-21.56Z"/></svg>
    }
}

#[island]
fn DialogMenu(children: Children) -> impl IntoView {
    let dialog_ref: NodeRef<leptos::html::Dialog> =
        NodeRef::new();

    view! {
        <button
            type="button"
            on:click={move |_| {
                let Some(dialog) = dialog_ref.get() else {
                    return;
                };
                dialog.show_modal();
            }}
            class="relative"
            aria-label="Open navigation"
        >
            <MenuIcon class="h-6 w-6 stroke-slate-500".to_string() />
        </button>
        <dialog
          node_ref=dialog_ref
          class="fixed inset-0 z-50 flex items-start overflow-y-auto bg-slate-900/50 pr-10 backdrop:backdrop-blur-sm lg:hidden backdrop:bg-slate-950/30 hidden open:flex"
          aria-label="Navigation"
        >
          <div class="min-h-full w-full max-w-xs bg-white px-4 pt-5 pb-12 sm:px-6 dark:bg-slate-900">
            <div class="flex items-center">
              <button
                type="button"
                on:click={move |_| {
                    let Some(dialog) = dialog_ref.get() else {
                        return;
                    };
                    dialog.close();
                  }}
                aria-label="Close navigation"
              >
                <CloseIcon class="h-6 w-6 stroke-slate-500".to_string() />
              </button>
              <a href="/" class="ml-6" aria-label="Home page">
                <Logo class="h-9 fill-slate-700 dark:fill-sky-100".to_string() />
              </a>
            </div>
            {children()}
          </div>
        </dialog>
    }
}
