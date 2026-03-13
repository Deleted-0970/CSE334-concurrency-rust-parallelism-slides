"""Pytest tests for python_to_rust module."""

import python_to_rust


def test_add():
    assert python_to_rust.add(1, 2) == 3


def test_fibonacci():
    assert python_to_rust.fibonacci(10) == 55


def test_process_string():
    assert python_to_rust.process_string("hello") == "OLLEH"
