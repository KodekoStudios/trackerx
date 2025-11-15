# **TrackerX — Universal Activity Tracker**

[![Crates.io Version](https://img.shields.io/crates/v/trackerx.svg)](https://crates.io/crates/trackerx)
[![Docs.rs](https://img.shields.io/docsrs/trackerx)](https://docs.rs/trackerx)

TrackerX is a compact, efficient, and fully offline command-line tool for recording and analyzing **any type of quantitative data** — expenses, productivity metrics, health information, project hours, or any other category you choose.

A single tool, unlimited use-cases.

---

# **Installation**

```bash
cargo install trackerx
```

---

# **Usage**

TrackerX uses **positional arguments** for a clean, predictable CLI.

---

## **Add Entry**

```bash
trackerx add <category> <value> [note]
```

Examples:

```bash
trackerx add money 14.99 "Steam — purchased game"
trackerx add calories 620 "lunch"
trackerx add work 3.5 "project A — planning session"
```

Duplicate entries for the same day may trigger a warning.

---

## **List Entries**

```bash
trackerx list [category] [days]
```

Examples:

```bash
trackerx list
trackerx list money
trackerx list work 7
```

Shows up to 50 recent entries, optionally filtered by category and/or time range.

---

## **Statistics**

```bash
trackerx stats <category>
```

Displays:

* total
* average
* minimum
* maximum
* number of entries

Example:

```bash
trackerx stats money
```

---

## **Categories**

```bash
trackerx categories
```

Lists all categories currently present in the database.

---

## **Remove Entries**

```bash
trackerx remove [id] [category] [days]
```

Examples:

```bash
trackerx remove 12              # remove by ID
trackerx remove money           # remove entire category
trackerx remove "" 7            # remove entries from last 7 days
trackerx remove "" money 30     # category + time window
```

If no entries match, TrackerX prints a clear message.

---

## **Export Data**

```bash
trackerx export <path>
```

Example:

```bash
trackerx export backup.json
```

Exports all entries as structured JSON.

---

# **Why TrackerX**

TrackerX is a general-purpose, extensible tracker. You can integrate it into **websites, dashboards, applications, or any workflow** — there are no limits.
It gives you full control over what you track and how you use the data.

---

# **Made with ❤️ by KodekoStudios**
