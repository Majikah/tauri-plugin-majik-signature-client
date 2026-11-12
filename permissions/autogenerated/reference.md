## Default Permission

Allows every majik-signature-client command.

#### This default permission set includes the following:

- `allow-open-session`
- `allow-get-snapshot`
- `allow-snapshot`
- `allow-hydrate`
- `allow-generate-mnemonic`
- `allow-create-account`
- `allow-import-account-from-mnemonic-backup`
- `allow-replace-account-from-mnemonic-backup`
- `allow-export-account-mnemonic-backup`
- `allow-add-own-account`
- `allow-remove-own-account`
- `allow-update-account-label`
- `allow-set-active-account`
- `allow-unlock-account`
- `allow-lock-account`
- `allow-lock-all-accounts`
- `allow-verify-passphrase`
- `allow-is-passphrase-valid`
- `allow-update-passphrase`
- `allow-replace-passphrase`
- `allow-ensure-identity-unlocked`
- `allow-has-signing-capability`
- `allow-reset-data`
- `allow-state-get`
- `allow-state-set`
- `allow-state-remove`
- `allow-state-list`
- `allow-keystore-export`
- `allow-keystore-restore`
- `allow-migrate-legacy-identities`
- `allow-begin-signing-session`
- `allow-end-signing-session`
- `allow-force-end-signing-session`
- `allow-is-signing-session-active`
- `allow-get-user-app-preferences`
- `allow-set-user-app-preferences`
- `allow-reset-user-app-preferences`
- `allow-list-history-logs`
- `allow-list-activity-logs`
- `allow-clear-history-logs`
- `allow-clear-activity-logs`
- `allow-restart-logs`
- `allow-hydrate-logs`
- `allow-update-own-account-meta`
- `allow-resolve-signer-label`
- `allow-list-contacts`
- `allow-get-contact`
- `allow-get-contact-by-fingerprint`
- `allow-get-contact-by-address`
- `allow-add-contact`
- `allow-remove-contact`
- `allow-update-contact-meta`
- `allow-import-contact-from-json`
- `allow-import-contact-from-string`
- `allow-export-contact-json`
- `allow-export-contact-string`
- `allow-clear-directory`
- `allow-set-majikah-status`
- `allow-block-contact`
- `allow-unblock-contact`
- `allow-list-groups`
- `allow-get-group`
- `allow-get-contacts-in-group`
- `allow-create-group`
- `allow-remove-group`
- `allow-update-group-meta`
- `allow-add-contact-to-group`
- `allow-add-contacts-to-group`
- `allow-remove-contact-from-group`
- `allow-move-contact-between-groups`
- `allow-add-contact-to-favorites`
- `allow-remove-contact-from-favorites`
- `allow-create-stamp`
- `allow-list-stamps`
- `allow-list-stamps-for-active-account`
- `allow-hydrate-stamps`
- `allow-lock-stamps`
- `allow-get-stamp`
- `allow-get-decrypted-stamp`
- `allow-decrypt-stamp-content`
- `allow-rename-stamp`
- `allow-replace-stamp-content`
- `allow-remove-stamp`
- `allow-inspect-stamp`
- `allow-is-stamped`
- `allow-sign-text`
- `allow-verify-text`
- `allow-get-signature-metadata`
- `allow-sign-file`
- `allow-sign-file-detached`
- `allow-resign-file`
- `allow-batch-sign-files`
- `allow-begin-sign-file-with-tsa`
- `allow-complete-sign-file-with-tsa`
- `allow-begin-sign-file-detached-with-tsa`
- `allow-complete-sign-file-detached-with-tsa`
- `allow-cancel-pending-tsa`
- `allow-verify-file`
- `allow-verify-file-all-signatures`
- `allow-verify-file-detached`
- `allow-verify-file-detached-all-signatures`
- `allow-verify-file-chain`
- `allow-verify-file-detached-chain`
- `allow-verify-file-revisions`
- `allow-verify-file-revisions-detached`
- `allow-extract-signatures`
- `allow-strip-signature`
- `allow-is-file-signed`
- `allow-get-allowlist`
- `allow-can-sign`
- `allow-is-multi-sig`
- `allow-get-signatories`
- `allow-get-issuer`
- `allow-get-envelope-info`
- `allow-get-file-signature-info`
- `allow-seal`
- `allow-verify-seal`
- `allow-get-seal-info`
- `allow-is-sealed`
- `allow-can-anchor`
- `allow-register-chain-anchor`
- `allow-describe-mjks-map`
- `allow-is-mjks-map`
- `allow-sign-batch-detached`
- `allow-cosign-mjks-map`
- `allow-verify-mjks-map`
- `allow-verify-file-order`
- `allow-verify-mjks-map-order`
- `allow-backup-contacts`
- `allow-backup-stamps`
- `allow-backup-app-data`
- `allow-probe-backup-type`
- `allow-read-app-data-backup`
- `allow-restore-contacts`
- `allow-restore-stamps`
- `allow-restore-app-data`
- `allow-restore-app-data-selective`

## Permission Table

<table>
<tr>
<th>Identifier</th>
<th>Description</th>
</tr>


<tr>
<td>

`majik-signature-client:allow-add-contact`

</td>
<td>

Enables the add_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-add-contact`

</td>
<td>

Denies the add_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-add-contact-to-favorites`

</td>
<td>

Enables the add_contact_to_favorites command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-add-contact-to-favorites`

</td>
<td>

Denies the add_contact_to_favorites command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-add-contact-to-group`

</td>
<td>

Enables the add_contact_to_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-add-contact-to-group`

</td>
<td>

Denies the add_contact_to_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-add-contacts-to-group`

</td>
<td>

Enables the add_contacts_to_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-add-contacts-to-group`

</td>
<td>

Denies the add_contacts_to_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-add-own-account`

</td>
<td>

Enables the add_own_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-add-own-account`

</td>
<td>

Denies the add_own_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-backup-app-data`

</td>
<td>

Enables the backup_app_data command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-backup-app-data`

</td>
<td>

Denies the backup_app_data command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-backup-contacts`

</td>
<td>

Enables the backup_contacts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-backup-contacts`

</td>
<td>

Denies the backup_contacts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-backup-stamps`

</td>
<td>

Enables the backup_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-backup-stamps`

</td>
<td>

Denies the backup_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-batch-sign-files`

</td>
<td>

Enables the batch_sign_files command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-batch-sign-files`

</td>
<td>

Denies the batch_sign_files command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-begin-sign-file-detached-with-tsa`

</td>
<td>

Enables the begin_sign_file_detached_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-begin-sign-file-detached-with-tsa`

</td>
<td>

Denies the begin_sign_file_detached_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-begin-sign-file-with-tsa`

</td>
<td>

Enables the begin_sign_file_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-begin-sign-file-with-tsa`

</td>
<td>

Denies the begin_sign_file_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-begin-signing-session`

</td>
<td>

Enables the begin_signing_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-begin-signing-session`

</td>
<td>

Denies the begin_signing_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-block-contact`

</td>
<td>

Enables the block_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-block-contact`

</td>
<td>

Denies the block_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-can-anchor`

</td>
<td>

Enables the can_anchor command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-can-anchor`

</td>
<td>

Denies the can_anchor command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-can-sign`

</td>
<td>

Enables the can_sign command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-can-sign`

</td>
<td>

Denies the can_sign command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-cancel-pending-tsa`

</td>
<td>

Enables the cancel_pending_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-cancel-pending-tsa`

</td>
<td>

Denies the cancel_pending_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-clear-activity-logs`

</td>
<td>

Enables the clear_activity_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-clear-activity-logs`

</td>
<td>

Denies the clear_activity_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-clear-directory`

</td>
<td>

Enables the clear_directory command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-clear-directory`

</td>
<td>

Denies the clear_directory command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-clear-history-logs`

</td>
<td>

Enables the clear_history_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-clear-history-logs`

</td>
<td>

Denies the clear_history_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-complete-sign-file-detached-with-tsa`

</td>
<td>

Enables the complete_sign_file_detached_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-complete-sign-file-detached-with-tsa`

</td>
<td>

Denies the complete_sign_file_detached_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-complete-sign-file-with-tsa`

</td>
<td>

Enables the complete_sign_file_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-complete-sign-file-with-tsa`

</td>
<td>

Denies the complete_sign_file_with_tsa command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-cosign-mjks-map`

</td>
<td>

Enables the cosign_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-cosign-mjks-map`

</td>
<td>

Denies the cosign_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-create-account`

</td>
<td>

Enables the create_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-create-account`

</td>
<td>

Denies the create_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-create-group`

</td>
<td>

Enables the create_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-create-group`

</td>
<td>

Denies the create_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-create-stamp`

</td>
<td>

Enables the create_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-create-stamp`

</td>
<td>

Denies the create_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-decrypt-stamp-content`

</td>
<td>

Enables the decrypt_stamp_content command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-decrypt-stamp-content`

</td>
<td>

Denies the decrypt_stamp_content command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-describe-mjks-map`

</td>
<td>

Enables the describe_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-describe-mjks-map`

</td>
<td>

Denies the describe_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-end-signing-session`

</td>
<td>

Enables the end_signing_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-end-signing-session`

</td>
<td>

Denies the end_signing_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-ensure-identity-unlocked`

</td>
<td>

Enables the ensure_identity_unlocked command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-ensure-identity-unlocked`

</td>
<td>

Denies the ensure_identity_unlocked command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-export-account-mnemonic-backup`

</td>
<td>

Enables the export_account_mnemonic_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-export-account-mnemonic-backup`

</td>
<td>

Denies the export_account_mnemonic_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-export-contact-json`

</td>
<td>

Enables the export_contact_json command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-export-contact-json`

</td>
<td>

Denies the export_contact_json command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-export-contact-string`

</td>
<td>

Enables the export_contact_string command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-export-contact-string`

</td>
<td>

Denies the export_contact_string command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-extract-signatures`

</td>
<td>

Enables the extract_signatures command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-extract-signatures`

</td>
<td>

Denies the extract_signatures command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-force-end-signing-session`

</td>
<td>

Enables the force_end_signing_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-force-end-signing-session`

</td>
<td>

Denies the force_end_signing_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-generate-mnemonic`

</td>
<td>

Enables the generate_mnemonic command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-generate-mnemonic`

</td>
<td>

Denies the generate_mnemonic command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-allowlist`

</td>
<td>

Enables the get_allowlist command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-allowlist`

</td>
<td>

Denies the get_allowlist command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-contact`

</td>
<td>

Enables the get_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-contact`

</td>
<td>

Denies the get_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-contact-by-address`

</td>
<td>

Enables the get_contact_by_address command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-contact-by-address`

</td>
<td>

Denies the get_contact_by_address command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-contact-by-fingerprint`

</td>
<td>

Enables the get_contact_by_fingerprint command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-contact-by-fingerprint`

</td>
<td>

Denies the get_contact_by_fingerprint command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-contacts-in-group`

</td>
<td>

Enables the get_contacts_in_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-contacts-in-group`

</td>
<td>

Denies the get_contacts_in_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-decrypted-stamp`

</td>
<td>

Enables the get_decrypted_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-decrypted-stamp`

</td>
<td>

Denies the get_decrypted_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-envelope-info`

</td>
<td>

Enables the get_envelope_info command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-envelope-info`

</td>
<td>

Denies the get_envelope_info command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-file-signature-info`

</td>
<td>

Enables the get_file_signature_info command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-file-signature-info`

</td>
<td>

Denies the get_file_signature_info command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-group`

</td>
<td>

Enables the get_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-group`

</td>
<td>

Denies the get_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-issuer`

</td>
<td>

Enables the get_issuer command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-issuer`

</td>
<td>

Denies the get_issuer command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-seal-info`

</td>
<td>

Enables the get_seal_info command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-seal-info`

</td>
<td>

Denies the get_seal_info command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-signatories`

</td>
<td>

Enables the get_signatories command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-signatories`

</td>
<td>

Denies the get_signatories command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-signature-metadata`

</td>
<td>

Enables the get_signature_metadata command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-signature-metadata`

</td>
<td>

Denies the get_signature_metadata command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-snapshot`

</td>
<td>

Enables the get_snapshot command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-snapshot`

</td>
<td>

Denies the get_snapshot command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-stamp`

</td>
<td>

Enables the get_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-stamp`

</td>
<td>

Denies the get_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-get-user-app-preferences`

</td>
<td>

Enables the get_user_app_preferences command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-get-user-app-preferences`

</td>
<td>

Denies the get_user_app_preferences command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-has-signing-capability`

</td>
<td>

Enables the has_signing_capability command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-has-signing-capability`

</td>
<td>

Denies the has_signing_capability command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-hydrate`

</td>
<td>

Enables the hydrate command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-hydrate`

</td>
<td>

Denies the hydrate command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-hydrate-logs`

</td>
<td>

Enables the hydrate_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-hydrate-logs`

</td>
<td>

Denies the hydrate_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-hydrate-stamps`

</td>
<td>

Enables the hydrate_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-hydrate-stamps`

</td>
<td>

Denies the hydrate_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-import-account-from-mnemonic-backup`

</td>
<td>

Enables the import_account_from_mnemonic_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-import-account-from-mnemonic-backup`

</td>
<td>

Denies the import_account_from_mnemonic_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-import-contact-from-json`

</td>
<td>

Enables the import_contact_from_json command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-import-contact-from-json`

</td>
<td>

Denies the import_contact_from_json command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-import-contact-from-string`

</td>
<td>

Enables the import_contact_from_string command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-import-contact-from-string`

</td>
<td>

Denies the import_contact_from_string command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-inspect-stamp`

</td>
<td>

Enables the inspect_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-inspect-stamp`

</td>
<td>

Denies the inspect_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-is-file-signed`

</td>
<td>

Enables the is_file_signed command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-is-file-signed`

</td>
<td>

Denies the is_file_signed command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-is-mjks-map`

</td>
<td>

Enables the is_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-is-mjks-map`

</td>
<td>

Denies the is_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-is-multi-sig`

</td>
<td>

Enables the is_multi_sig command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-is-multi-sig`

</td>
<td>

Denies the is_multi_sig command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-is-passphrase-valid`

</td>
<td>

Enables the is_passphrase_valid command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-is-passphrase-valid`

</td>
<td>

Denies the is_passphrase_valid command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-is-sealed`

</td>
<td>

Enables the is_sealed command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-is-sealed`

</td>
<td>

Denies the is_sealed command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-is-signing-session-active`

</td>
<td>

Enables the is_signing_session_active command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-is-signing-session-active`

</td>
<td>

Denies the is_signing_session_active command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-is-stamped`

</td>
<td>

Enables the is_stamped command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-is-stamped`

</td>
<td>

Denies the is_stamped command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-keystore-export`

</td>
<td>

Enables the keystore_export command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-keystore-export`

</td>
<td>

Denies the keystore_export command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-keystore-restore`

</td>
<td>

Enables the keystore_restore command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-keystore-restore`

</td>
<td>

Denies the keystore_restore command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-list-activity-logs`

</td>
<td>

Enables the list_activity_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-list-activity-logs`

</td>
<td>

Denies the list_activity_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-list-contacts`

</td>
<td>

Enables the list_contacts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-list-contacts`

</td>
<td>

Denies the list_contacts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-list-groups`

</td>
<td>

Enables the list_groups command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-list-groups`

</td>
<td>

Denies the list_groups command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-list-history-logs`

</td>
<td>

Enables the list_history_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-list-history-logs`

</td>
<td>

Denies the list_history_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-list-stamps`

</td>
<td>

Enables the list_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-list-stamps`

</td>
<td>

Denies the list_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-list-stamps-for-active-account`

</td>
<td>

Enables the list_stamps_for_active_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-list-stamps-for-active-account`

</td>
<td>

Denies the list_stamps_for_active_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-lock-account`

</td>
<td>

Enables the lock_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-lock-account`

</td>
<td>

Denies the lock_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-lock-all-accounts`

</td>
<td>

Enables the lock_all_accounts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-lock-all-accounts`

</td>
<td>

Denies the lock_all_accounts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-lock-stamps`

</td>
<td>

Enables the lock_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-lock-stamps`

</td>
<td>

Denies the lock_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-migrate-legacy-identities`

</td>
<td>

Enables the migrate_legacy_identities command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-migrate-legacy-identities`

</td>
<td>

Denies the migrate_legacy_identities command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-move-contact-between-groups`

</td>
<td>

Enables the move_contact_between_groups command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-move-contact-between-groups`

</td>
<td>

Denies the move_contact_between_groups command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-open-session`

</td>
<td>

Enables the open_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-open-session`

</td>
<td>

Denies the open_session command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-probe-backup-type`

</td>
<td>

Enables the probe_backup_type command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-probe-backup-type`

</td>
<td>

Denies the probe_backup_type command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-read-app-data-backup`

</td>
<td>

Enables the read_app_data_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-read-app-data-backup`

</td>
<td>

Denies the read_app_data_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-register-chain-anchor`

</td>
<td>

Enables the register_chain_anchor command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-register-chain-anchor`

</td>
<td>

Denies the register_chain_anchor command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-remove-contact`

</td>
<td>

Enables the remove_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-remove-contact`

</td>
<td>

Denies the remove_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-remove-contact-from-favorites`

</td>
<td>

Enables the remove_contact_from_favorites command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-remove-contact-from-favorites`

</td>
<td>

Denies the remove_contact_from_favorites command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-remove-contact-from-group`

</td>
<td>

Enables the remove_contact_from_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-remove-contact-from-group`

</td>
<td>

Denies the remove_contact_from_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-remove-group`

</td>
<td>

Enables the remove_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-remove-group`

</td>
<td>

Denies the remove_group command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-remove-own-account`

</td>
<td>

Enables the remove_own_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-remove-own-account`

</td>
<td>

Denies the remove_own_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-remove-stamp`

</td>
<td>

Enables the remove_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-remove-stamp`

</td>
<td>

Denies the remove_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-rename-stamp`

</td>
<td>

Enables the rename_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-rename-stamp`

</td>
<td>

Denies the rename_stamp command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-replace-account-from-mnemonic-backup`

</td>
<td>

Enables the replace_account_from_mnemonic_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-replace-account-from-mnemonic-backup`

</td>
<td>

Denies the replace_account_from_mnemonic_backup command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-replace-passphrase`

</td>
<td>

Enables the replace_passphrase command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-replace-passphrase`

</td>
<td>

Denies the replace_passphrase command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-replace-stamp-content`

</td>
<td>

Enables the replace_stamp_content command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-replace-stamp-content`

</td>
<td>

Denies the replace_stamp_content command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-reset-data`

</td>
<td>

Enables the reset_data command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-reset-data`

</td>
<td>

Denies the reset_data command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-reset-user-app-preferences`

</td>
<td>

Enables the reset_user_app_preferences command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-reset-user-app-preferences`

</td>
<td>

Denies the reset_user_app_preferences command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-resign-file`

</td>
<td>

Enables the resign_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-resign-file`

</td>
<td>

Denies the resign_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-resolve-signer-label`

</td>
<td>

Enables the resolve_signer_label command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-resolve-signer-label`

</td>
<td>

Denies the resolve_signer_label command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-restart-logs`

</td>
<td>

Enables the restart_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-restart-logs`

</td>
<td>

Denies the restart_logs command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-restore-app-data`

</td>
<td>

Enables the restore_app_data command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-restore-app-data`

</td>
<td>

Denies the restore_app_data command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-restore-app-data-selective`

</td>
<td>

Enables the restore_app_data_selective command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-restore-app-data-selective`

</td>
<td>

Denies the restore_app_data_selective command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-restore-contacts`

</td>
<td>

Enables the restore_contacts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-restore-contacts`

</td>
<td>

Denies the restore_contacts command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-restore-stamps`

</td>
<td>

Enables the restore_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-restore-stamps`

</td>
<td>

Denies the restore_stamps command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-seal`

</td>
<td>

Enables the seal command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-seal`

</td>
<td>

Denies the seal command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-set-active-account`

</td>
<td>

Enables the set_active_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-set-active-account`

</td>
<td>

Denies the set_active_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-set-majikah-status`

</td>
<td>

Enables the set_majikah_status command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-set-majikah-status`

</td>
<td>

Denies the set_majikah_status command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-set-user-app-preferences`

</td>
<td>

Enables the set_user_app_preferences command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-set-user-app-preferences`

</td>
<td>

Denies the set_user_app_preferences command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-sign-batch-detached`

</td>
<td>

Enables the sign_batch_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-sign-batch-detached`

</td>
<td>

Denies the sign_batch_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-sign-file`

</td>
<td>

Enables the sign_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-sign-file`

</td>
<td>

Denies the sign_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-sign-file-detached`

</td>
<td>

Enables the sign_file_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-sign-file-detached`

</td>
<td>

Denies the sign_file_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-sign-text`

</td>
<td>

Enables the sign_text command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-sign-text`

</td>
<td>

Denies the sign_text command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-snapshot`

</td>
<td>

Enables the snapshot command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-snapshot`

</td>
<td>

Denies the snapshot command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-state-get`

</td>
<td>

Enables the state_get command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-state-get`

</td>
<td>

Denies the state_get command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-state-list`

</td>
<td>

Enables the state_list command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-state-list`

</td>
<td>

Denies the state_list command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-state-remove`

</td>
<td>

Enables the state_remove command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-state-remove`

</td>
<td>

Denies the state_remove command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-state-set`

</td>
<td>

Enables the state_set command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-state-set`

</td>
<td>

Denies the state_set command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-strip-signature`

</td>
<td>

Enables the strip_signature command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-strip-signature`

</td>
<td>

Denies the strip_signature command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-unblock-contact`

</td>
<td>

Enables the unblock_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-unblock-contact`

</td>
<td>

Denies the unblock_contact command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-unlock-account`

</td>
<td>

Enables the unlock_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-unlock-account`

</td>
<td>

Denies the unlock_account command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-update-account-label`

</td>
<td>

Enables the update_account_label command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-update-account-label`

</td>
<td>

Denies the update_account_label command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-update-contact-meta`

</td>
<td>

Enables the update_contact_meta command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-update-contact-meta`

</td>
<td>

Denies the update_contact_meta command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-update-group-meta`

</td>
<td>

Enables the update_group_meta command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-update-group-meta`

</td>
<td>

Denies the update_group_meta command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-update-own-account-meta`

</td>
<td>

Enables the update_own_account_meta command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-update-own-account-meta`

</td>
<td>

Denies the update_own_account_meta command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-update-passphrase`

</td>
<td>

Enables the update_passphrase command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-update-passphrase`

</td>
<td>

Denies the update_passphrase command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file`

</td>
<td>

Enables the verify_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file`

</td>
<td>

Denies the verify_file command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-all-signatures`

</td>
<td>

Enables the verify_file_all_signatures command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-all-signatures`

</td>
<td>

Denies the verify_file_all_signatures command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-chain`

</td>
<td>

Enables the verify_file_chain command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-chain`

</td>
<td>

Denies the verify_file_chain command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-detached`

</td>
<td>

Enables the verify_file_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-detached`

</td>
<td>

Denies the verify_file_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-detached-all-signatures`

</td>
<td>

Enables the verify_file_detached_all_signatures command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-detached-all-signatures`

</td>
<td>

Denies the verify_file_detached_all_signatures command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-detached-chain`

</td>
<td>

Enables the verify_file_detached_chain command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-detached-chain`

</td>
<td>

Denies the verify_file_detached_chain command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-order`

</td>
<td>

Enables the verify_file_order command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-order`

</td>
<td>

Denies the verify_file_order command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-revisions`

</td>
<td>

Enables the verify_file_revisions command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-revisions`

</td>
<td>

Denies the verify_file_revisions command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-file-revisions-detached`

</td>
<td>

Enables the verify_file_revisions_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-file-revisions-detached`

</td>
<td>

Denies the verify_file_revisions_detached command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-mjks-map`

</td>
<td>

Enables the verify_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-mjks-map`

</td>
<td>

Denies the verify_mjks_map command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-mjks-map-order`

</td>
<td>

Enables the verify_mjks_map_order command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-mjks-map-order`

</td>
<td>

Denies the verify_mjks_map_order command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-passphrase`

</td>
<td>

Enables the verify_passphrase command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-passphrase`

</td>
<td>

Denies the verify_passphrase command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-seal`

</td>
<td>

Enables the verify_seal command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-seal`

</td>
<td>

Denies the verify_seal command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:allow-verify-text`

</td>
<td>

Enables the verify_text command without any pre-configured scope.

</td>
</tr>

<tr>
<td>

`majik-signature-client:deny-verify-text`

</td>
<td>

Denies the verify_text command without any pre-configured scope.

</td>
</tr>
</table>
