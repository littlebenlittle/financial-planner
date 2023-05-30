---
id: 2CGWuxNOS6NWKNWxyj
date: 2023-05-20
title: Exploring the Tool
template: post
---

# Exploring the Tool

### Quick Summary

> The first goal when using a new tool is to explore what you can do with the
  tool. This is not an intrinsic propery of the tool, but rather a harmony between
  tool and person. What another person might do with that tool is irrelevant--what
  can **you** do with it?

## The Tool: Rust and Yew

Yew is a reactive framework for the rust programming language. It allows
developers to make use of rust's powerful static type checker to build robust
front-end applications. The explicit memory management of rust can help identify
and debug performance problems that may be obfuscated by other frontend
frameworks written in JavaScript.

Building the financial planner application with rust and Yew started with
exploring what these tools can do. After some fiddling, I was able to create
a UI that enables the user to submit income and expense forms that are then
recorded in the transaction log. I also configured a histogram that plotted daily
income and expnses in a given date range.

At this point the app was mostly doing what I expected with a few hiccups. These
hiccups are when I really started to need to think about *what* each compoment
of the UI was supposed to do and how it interacts with the computations being
performed in the background.
