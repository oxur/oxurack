# Iteration00 preservation

CRC preserved these bytes from planning commit
`2eb5ac138fedf21d4185d1cc2f1dd3df84c4972d` before issuing iteration01 on
2026-10-10. Original paths are the Slice02 root for closing-report/ledger and
Slice02 artifacts for the other files. Original raw logs remain under
`../logs/`; the issued `../../cc-prompt.md` remains unchanged.

These are historical CC attestations/authoring records, not newly accepted
evidence. The copied verifier is reference-only (its relative root depended
on its original location); use the current `../verify-planning.py` to validate
the active packet. The canonical closing report is still the initial return
until CC writes its actual iteration01 return.
