//! # tauri-plugin-majik-signature-client
//!
//! Exposes [`majik_signature_client`] to the webview as `plugin:majik-signature-client|*` commands
//! and a `majik-signature-client:event` stream. It is a superset of the key client plugin: the
//! account commands have the same names, so `@majikah/majik-signature-client-tauri` extends the key
//! client's TypeScript facade.
//!
//! File arguments are paths (Rust reads and validates them) or small inline base64 blobs; results
//! are written to a path or returned inline. See `majik_signature_client::ipc::input`.
//!
//! ```ignore
//! // inside .setup(|app| { ... }) where `conn` is the app's shared SQLCipher connection:
//! app.handle().plugin(tauri_plugin_majik_signature_client::init(conn.clone()))?;
//! ```
//! and add `"majik-signature-client:default"` to the window capability.

mod commands;
mod handle;

pub use handle::SigHandle;

use majik_signature_client::{SharedConnection, SignatureClient};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, RunEvent, Runtime};

pub const PLUGIN_NAME: &str = "majik-signature-client";

/// `conn` is the app's shared connection; the plugin creates the key, state, contact, group,
/// stamp and log tables if missing. The client is hydrated by the first `open_session` call.
pub fn init<R: Runtime>(conn: SharedConnection) -> TauriPlugin<R> {
    Builder::<R>::new(PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            commands::open_session,
            commands::get_snapshot,
            commands::snapshot,
            commands::hydrate,
            commands::generate_mnemonic,
            commands::create_account,
            commands::import_account_from_mnemonic_backup,
            commands::replace_account_from_mnemonic_backup,
            commands::export_account_mnemonic_backup,
            commands::add_own_account,
            commands::remove_own_account,
            commands::update_account_label,
            commands::set_active_account,
            commands::unlock_account,
            commands::lock_account,
            commands::lock_all_accounts,
            commands::verify_passphrase,
            commands::is_passphrase_valid,
            commands::update_passphrase,
            commands::replace_passphrase,
            commands::ensure_identity_unlocked,
            commands::has_signing_capability,
            commands::reset_data,
            commands::state_get,
            commands::state_set,
            commands::state_remove,
            commands::state_list,
            commands::keystore_export,
            commands::keystore_restore,
            commands::migrate_legacy_identities,
            commands::begin_signing_session,
            commands::end_signing_session,
            commands::force_end_signing_session,
            commands::is_signing_session_active,
            commands::get_user_app_preferences,
            commands::set_user_app_preferences,
            commands::reset_user_app_preferences,
            commands::list_history_logs,
            commands::list_activity_logs,
            commands::clear_history_logs,
            commands::clear_activity_logs,
            commands::restart_logs,
            commands::hydrate_logs,
            commands::update_own_account_meta,
            commands::resolve_signer_label,
            commands::list_contacts,
            commands::get_contact,
            commands::get_contact_by_fingerprint,
            commands::get_contact_by_address,
            commands::add_contact,
            commands::remove_contact,
            commands::update_contact_meta,
            commands::import_contact_from_json,
            commands::import_contact_from_string,
            commands::export_contact_json,
            commands::export_contact_string,
            commands::clear_directory,
            commands::set_majikah_status,
            commands::block_contact,
            commands::unblock_contact,
            commands::list_groups,
            commands::get_group,
            commands::get_contacts_in_group,
            commands::create_group,
            commands::remove_group,
            commands::update_group_meta,
            commands::add_contact_to_group,
            commands::add_contacts_to_group,
            commands::remove_contact_from_group,
            commands::move_contact_between_groups,
            commands::add_contact_to_favorites,
            commands::remove_contact_from_favorites,
            commands::create_stamp,
            commands::list_stamps,
            commands::list_stamps_for_active_account,
            commands::hydrate_stamps,
            commands::lock_stamps,
            commands::get_stamp,
            commands::get_decrypted_stamp,
            commands::decrypt_stamp_content,
            commands::rename_stamp,
            commands::replace_stamp_content,
            commands::remove_stamp,
            commands::inspect_stamp,
            commands::is_stamped,
            commands::sign_text,
            commands::verify_text,
            commands::get_signature_metadata,
            commands::sign_file,
            commands::sign_file_detached,
            commands::resign_file,
            commands::batch_sign_files,
            commands::begin_sign_file_with_tsa,
            commands::complete_sign_file_with_tsa,
            commands::begin_sign_file_detached_with_tsa,
            commands::complete_sign_file_detached_with_tsa,
            commands::cancel_pending_tsa,
            commands::verify_file,
            commands::verify_file_all_signatures,
            commands::verify_file_detached,
            commands::verify_file_detached_all_signatures,
            commands::verify_file_chain,
            commands::verify_file_detached_chain,
            commands::verify_file_revisions,
            commands::verify_file_revisions_detached,
            commands::extract_signatures,
            commands::strip_signature,
            commands::is_file_signed,
            commands::get_allowlist,
            commands::can_sign,
            commands::is_multi_sig,
            commands::get_signatories,
            commands::get_issuer,
            commands::get_envelope_info,
            commands::get_file_signature_info,
            commands::seal,
            commands::verify_seal,
            commands::get_seal_info,
            commands::is_sealed,
            commands::can_anchor,
            commands::register_chain_anchor,
            commands::describe_mjks_map,
            commands::is_mjks_map,
            commands::sign_batch_detached,
            commands::cosign_mjks_map,
            commands::verify_mjks_map,
            commands::verify_file_order,
            commands::verify_mjks_map_order,
            commands::backup_contacts,
            commands::backup_stamps,
            commands::backup_app_data,
            commands::probe_backup_type,
            commands::read_app_data_backup,
            commands::restore_contacts,
            commands::restore_stamps,
            commands::restore_app_data,
            commands::restore_app_data_selective,
        ])
        .setup(move |app, _api| {
            let client = SignatureClient::with_sqlite(conn)?;
            let handle = SigHandle::new(client);
            handle.forward_events(app.clone());
            app.manage(handle);
            Ok(())
        })
        .on_event(|app, event| {
            // Zeroize unlocked keys and decrypted stamps on the way out.
            if matches!(event, RunEvent::ExitRequested { .. } | RunEvent::Exit) {
                if let Some(handle) = app.try_state::<SigHandle>() {
                    handle.lock_all_blocking();
                }
            }
        })
        .build()
}
