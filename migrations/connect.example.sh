#!/bin/sh

PGPASSWORD=postgres psql -h localhost -p 5432 -d postgres -U postgres
