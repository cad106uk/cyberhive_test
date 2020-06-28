ssh -L 7777:127.0.0.1:6969 serve@87.254.5.232 -N -f

python3 -m venv client_venv
source client_venv/bin/activate

pushd ..
python3 pip install -r requirements.txt
python3 src/main.py
popd
deactivate
