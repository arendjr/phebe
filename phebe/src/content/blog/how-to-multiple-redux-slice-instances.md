---
title: "Advanced Redux: How to create multiple instances of a state slice"
description: "This post describes a technique for creating multiple instances of a Redux state slice within the same store."
pubDate: "Jan 31 2021"
---

This post describes a technique for creating multiple instances of a Redux state slice within the same store. I believe this technique can be useful if your application deals with multiple tabs or documents.

**Heads-up:** This post assumes prior experience with Redux and is aimed at medium to expert-level engineers.

## Introduction

When you use Redux, you should be familiar with the concept of a state slice, sometimes referred to as a subtree. They are a part of your state tree that is managed by its own reducer. Commonly, people use `combineReducers()` to create their root reducer from smaller ones; one per slice.

### Example

In this example we only have a single slice, todos:

```ts
const reducer = combineReducers({ todos: todosReducer });
```

But what if we want to have multiple tabs in our application, where every tab has its own TODO list?

A naive approach would be to make `todosReducer` aware of the fact we support multiple tabs and let its state slice reflect this. Unfortunately, this becomes unwieldy fast as every TODO action needs to have an identifier for the right tab in its payload and the reducer needs to account for this in how it handles each and every action. The problem becomes even bigger when besides `todos`, other slices need to account for the tabs as well.

So ideally, we would be able to compose our reducers in such a way that we can use our `todosReducer`, and possibly any other reducers, unaltered.

Fortunately, we can! It takes a little bit more effort than simply calling a helper like `combineReducers()`, so I'll show you how to do this step-by-step.

## 1. Define the state slice for the tabs

Using TypeScript, we define the types for the state slice. There will be two types: one to keep the state per-tab and one to keep the state of which tabs are open.

```ts
type Tab = {
  readonly id: string;
  readonly todos: TodosState;
};

type TabsState = {
  readonly activeTabId: string | null;
  readonly tabs: { [id: string]: Tab };
};
```

As you can see, the state per-tab simply contains the `todos` slice, plus the ID of the tab.

The `TabsState` type then simply tracks the tabs by ID. And it decides which of them is active.

Now that we know what the tabs slice looks like, we can define its `initialState`:

```ts
const initialState: TabsState = {
  activeTabId: null,
  tabs: {},
};
```

## 2. Create action wrapper

Before we move on to the reducer, we will first create an action wrapper. What is an action wrapper, you ask? It's an action creator that wraps another action. Like this:

```ts
const WITH_TAB = "WITH_TAB";

type WrappedTabAction = {
  type: typeof WITH_TAB;
  payload: { action: TodoAction; tabId: string };
};

function withTab(
  tabId: string,
  action: TodoAction
): WrappedTabAction {
  return { type: WITH_TAB, payload: { action, tabId } };
}
```

So now if we have an `addTodo(task)` action creator, we could use it like this to add the TODO to the correct tab: `dispatch(withTab(tabId, addTodo("Hello")))`

Does it look like a hassle to wrap all your action creators like that? Don't worry, we'll get back to this and have a look on how to make this more ergonomic later.

By the way, did you notice we wrapped `TodoAction` in the code above? If there were multiple slices inside a tab, that would be replaced with a `TabAction` type that would be a union over all the action types that can be wrapped.

## 3. Create the reducer

With all that in place, we can now create our reducer. It will be split into two functions, which correspond to the two types we created earlier:

```ts
function tabReducer(state: Tab, action: TabAction): Tab {
  const todos = todosReducer(state.todos, action);
  if (todos === state.todos) {
    return state;
  } else {
    return { ...state, todos };
  }
}

function tabsReducer(
  state = initialState,
  action: AppAction
): TabsState {
  switch (action.type) {
    case WITH_TAB: {
      const {
        action: unwrappedAction,
        tabId
      } = action.payload;

      let changed = false;
      const tabs: { [id: string]: Tab } = {};
      for (const [id, tab] of Object.entries(state.tabs)) {
        if (id === tabId) {
          const newTab = tabReducer(tab, unwrappedAction);
          if (newTab !== tab) {
            changed = true;
          }

          tabs[id] = newTab;
        } else {
          tabs[id] = tab;
        }
      }

      if (changed) {
        return { ...state, tabs };
      } else {
        return state;
      }
    }
  default:
    return state;
  }
}
```

The first function simply delegates to the `todoReducer`. Note how it returns the original state object if there were no changes. This is important to maintain identity, so that selectors don't need to be re-evaluated.

The second function is the heart of this entire technique, you might say. Whenever a wrapped action comes along, it unwraps it and goes through the tabs to apply it to the correct one. Once again, it checks to see if there are changes, so that it can maintain identity if there aren't.

Finally, note the reference to an `AppAction` in the signature of `tabsReducer`. This is expected to be a union over all the action types that may be dispatched in your app. If you don't have a defined type for that, you could replace it with `AnyAction`, but you may need to explicitly cast action inside the case branch then. In any case, I do advise you to have such a type for reasons I'll discuss below.

## 4. Update your selectors

Now there is only one piece of the puzzle left: your selectors. It's likely you already had a selector for your TODO state:

```ts
const selectTodos = (state: RootState) => state.todos;
```

This selector wouldn't work anymore, because todos is no longer part of your root state. Instead, it is found under your tabs now. So how to update the selector?

First, we'll create a selector for retrieving the active tab using Reselect's `createSelector()`:

```ts
const selectTabs = (state: RootState) => state.tabs;

const selectActiveTab = createSelector(
  [selectTabs],
  (state) => {
    const { activeTabId } = state;
    const activeTab =
      activeTabId && state.tabs[activeTabId];
    if (!activeTab) {
      throw new Error("No active tab");
    }
    return activeTab;
  }
);
```

As you can see, this selector throws whenever there is no active tab. As a result, you should only use it inside components that live inside a tab, because that will guarantee there is an active tab. If that is an acceptable trade-off for you, you're probably happy to leave this exception in. The alternative is to allow the selector to return undefined, which means you will need to handle that case everywhere you use the selector.

And now your TODO selector can simply be rewritten to become:

```ts
const selectTodos = createSelector(
  [selectActiveTab],
  (tab) => tab.todos
);
```

## Bonus: Dispatch hooks

The foundation is there now, but we are still lacking in the ergonomics department. Remember how every time we want to dispatch a tab action, we need to wrap it like this, `dispatch(withTab(tabId, addTodo("Hello")))`?

There are two problems with this. First of which, it requires us to know the tabId, which means we often need to use a selector just for that purpose. That's going to get inconvenient fast, so what we may want to do is the following: we can create a new action wrapper called `withActiveTab()`, which works the same as `withTab()` except it doesn't require a `tabId` in the payload, because the reducer already knows which tab that is anyway. If you think this is something you want, I trust you can make the necessary adjustments to the reducer on your own.

The other problem is that it is easy to make mistakes. After all, which actions should be wrapped, and which ones shouldn't? Now, remember we briefly touched upon those `AppAction` and `TabAction` types? This is where those types become valuable. Because if we have a closed `TabAction` type that is a union over all the action types for slices that are under your tabs slice, your `withTab()` wrapper will reject any action that is not explicitly a part of that union. Similarly, if we have a closed `AppAction` type, which should explicitly not contain the tab actions, and we use that type in the signature of our `dispatch()` function, then we cannot accidentally dispatch tab actions without wrapping them first.

So knowing all this, we can define two React hooks for giving us two separate dispatch functions, one for regular "app" dispatching, and one for "tab" dispatching. This way, we get both convenience and type safety:

```ts
type AppDispatch = (action: AppAction) => void;

type TabDispatch = (action: TabAction) => void;

const dispatch = store.dispatch as AppDispatch;
    
const useAppDispatch = () => dispatch;
const useActiveTabDispatch = () => (action: TabAction) =>
  dispatch(withActiveTab(action));
```

## Wrapping up

At this point you should have an understanding of all the pieces necessary to create multiple instances of state slices in your own store: the types you need, how to wrap actions, the reducer implementation and the changes to your selectors. And with the custom hooks, its usage is even reasonably ergonomic.

Now, there are a few things missing still, which I will leave as an exercise to the reader:

- How to display the tabs is entirely up to your own application. When you do try to build this, you might find that you want to be explicit about the ordering of the tabs as well, something we didn't consider so far.
- Actions for manipulating the tabs, such as opening, closing and selecting tabs have not yet been implemented.
- You may want to extend our custom dispatchers so that they can dispatch thunks as well.
