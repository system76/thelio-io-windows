#!/usr/bin/env python3

import argparse
import os
import shutil
import subprocess
import urllib.request
from zipfile import ZipFile

# Handle commandline arguments
parser = argparse.ArgumentParser()
parser.add_argument('--sign', action='store_true')
args = parser.parse_args()

# Build .msi
subprocess.check_call([
    "cargo",
    "wix",
    "--nocapture",
    "--verbose",
])

if args.sign:
    if not os.path.isdir('target/sign'):
        os.mkdir("target/sign")

    # Download signing tool
    tool_url = "https://www.ssl.com/download/29773/"
    tool_zip = "target/sign/CodeSignTool.zip"
    if not os.path.isfile(tool_zip):
        if os.path.isfile(tool_zip + ".partial"):
            os.remove(tool_zip + ".partial")
        urllib.request.urlretrieve(tool_url, tool_zip + ".partial")
        os.rename(tool_zip + ".partial", tool_zip)

    # Extract signing tool
    tool_dir = "target/sign/CodeSignTool"
    if not os.path.isdir(tool_dir):
        if os.path.isdir(tool_dir + ".partial"):
            shutil.rmtree(tool_dir + ".partial")
        os.mkdir(tool_dir + ".partial")
        with ZipFile(tool_zip, "r") as zip:
            zip.extractall(tool_dir + ".partial")
        os.rename(tool_dir + ".partial", tool_dir)

    # Sign with specified cloud signing key
    subprocess.check_call([
        "cmd", "/c", "CodeSignTool.bat",
        "sign",
        "-credential_id=" + os.environ["SSL_COM_CREDENTIAL_ID"],
        "-username=" + os.environ["SSL_COM_USERNAME"],
        "-password=" + os.environ["SSL_COM_PASSWORD"],
        "-totp_secret=" + os.environ["SSL_COM_TOTP_SECRET"],
        "-program_name=System76 Thelio Io",
        "-input_file_path=../../../wix/thelio-io-0.1.0-x86_64.msi",
        "-output_dir_path=../../",
    ], cwd="target/sign/CodeSignTool/CodeSignTool-v1.1.0-windows")
