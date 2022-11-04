#!/bin/bash
ps -ef|grep rim|grep -v 'grep'| awk '{print $2}' |  xargs kill -9