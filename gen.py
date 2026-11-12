import json
from pathlib import Path

# Single source of truth for the plugin's commands. Run: python3 gen.py
KEY = "majik_signature_client::majik_key_client"
SIG = "majik_signature_client"
CON = "majik_signature_client::majik_contact_manager::majik_contact"
SGN = "majik_signature_client::majik_signature"

# (name, params, return type, body)  -- body runs inside `state.run(move |c| BODY)` with c: &mut SignatureClient
C = []
def cmd(name, params, ret, body): C.append((name, params, ret, body))

SECRET = "Zeroizing<String>"

# ── session & account (key-client commands, same names as the key plugin) ──
cmd("open_session", "", f"ops::SigOpenedSession", "ops::open_session(c)")
cmd("get_snapshot", "", f"{KEY}::ipc::Snapshot", "Ok(ops::snapshot(c))")
# Compatibility alias used by MajikSignatureClientTauri.snapshotSignatureClient().
cmd("snapshot", "", f"{KEY}::ipc::Snapshot", "Ok(ops::snapshot(c))")
cmd("hydrate", "", f"{SIG}::HydrateReport", "ops::hydrate(c)")
cmd("generate_mnemonic", f"strength: u32, language: Option<{KEY}::MnemonicLanguage>", "String",
    f"Ok({KEY}::ipc::generate_mnemonic(c.keys(), strength, language)?)")
cmd("create_account", f"args: {KEY}::ipc::CreateAccountArgs", f"{KEY}::CreatedAccount",
    f"Ok({KEY}::ipc::create_account(c.keys_mut(), &args)?)")
cmd("import_account_from_mnemonic_backup", f"args: {KEY}::ipc::BackupAccountArgs", f"{KEY}::ImportedAccount",
    f"Ok({KEY}::ipc::import_account_from_mnemonic_backup(c.keys_mut(), &args)?)")
cmd("replace_account_from_mnemonic_backup", f"args: {KEY}::ipc::BackupAccountArgs", f"{KEY}::ImportedAccount",
    f"Ok({KEY}::ipc::replace_account_from_mnemonic_backup(c.keys_mut(), &args)?)")
cmd("export_account_mnemonic_backup", f"id: String, mnemonic: {SECRET}", "String",
    f"Ok({KEY}::ipc::export_account_mnemonic_backup(c.keys(), &id, &mnemonic)?)")
cmd("add_own_account", f"account: {CON}::SerializedMajikContact", "()",
    f"Ok({KEY}::ipc::add_own_account(c.keys_mut(), account)?)")
cmd("remove_own_account", "id: String", "bool", f"Ok({KEY}::ipc::remove_own_account(c.keys_mut(), &id)?)")
cmd("update_account_label", "id: String, label: String", "()", "ops::update_account_label(c, &id, &label)")
cmd("set_active_account", "id: String, bypass_identity: Option<bool>", "bool",
    f"Ok({KEY}::ipc::set_active_account(c.keys_mut(), &id, bypass_identity.unwrap_or(false))?)")
cmd("unlock_account", f"id: String, passphrase: {SECRET}", "()",
    f"Ok({KEY}::ipc::unlock_account(c.keys_mut(), &id, &passphrase)?)")
cmd("lock_account", "id: String", "()",
    f"{{ {KEY}::ipc::lock_account(c.keys_mut(), &id); let _ = c.lock_stamps(); Ok(()) }}")
cmd("lock_all_accounts", "", "()",
    f"{{ {KEY}::ipc::lock_all_accounts(c.keys_mut()); let _ = c.lock_stamps(); Ok(()) }}")
cmd("verify_passphrase", f"id: String, passphrase: {SECRET}", "bool",
    f"Ok({KEY}::ipc::verify_passphrase(c.keys_mut(), &id, &passphrase)?)")
cmd("is_passphrase_valid", f"passphrase: {SECRET}, id: Option<String>", "bool",
    f"Ok({KEY}::ipc::is_passphrase_valid(c.keys_mut(), &passphrase, id.as_deref())?)")
cmd("update_passphrase", f"args: {KEY}::ipc::UpdatePassphraseArgs", "()",
    f"Ok({KEY}::ipc::update_passphrase(c.keys_mut(), &args)?)")
cmd("replace_passphrase", f"args: {KEY}::ipc::ReplacePassphraseArgs", "String",
    f"Ok({KEY}::ipc::replace_passphrase(c.keys_mut(), &args)?)")
cmd("ensure_identity_unlocked", "id: String", "()", f"Ok({KEY}::ipc::ensure_identity_unlocked(c.keys_mut(), &id)?)")
cmd("has_signing_capability", "id: Option<String>", "bool",
    f"Ok({KEY}::ipc::has_signing_capability(c.keys(), id.as_deref()))")
cmd("reset_data", "", "()", "ops::reset_data(c)")
cmd("state_get", "key: String", "Option<String>", f"Ok({KEY}::ipc::state_get(c.keys_mut(), &key)?)")
cmd("state_set", "key: String, value: String", "()", f"Ok({KEY}::ipc::state_set(c.keys_mut(), &key, &value)?)")
cmd("state_remove", "key: String", "bool", f"Ok({KEY}::ipc::state_remove(c.keys_mut(), &key)?)")
cmd("state_list", "", f"Vec<{KEY}::ipc::StateEntryDto>", f"Ok({KEY}::ipc::state_list(c.keys()))")
cmd("keystore_export", "", f"Vec<{KEY}::MajikKeyJson>", f"Ok({KEY}::ipc::keystore_export(c.keys())?)")
cmd("keystore_restore", f"keys: Vec<{KEY}::MajikKeyJson>", f"{KEY}::HydrateReport",
    f"Ok({KEY}::ipc::keystore_restore(c.keys_mut(), keys)?)")
cmd("migrate_legacy_identities", f"identities: Vec<{KEY}::SerializedIdentity>", f"{KEY}::MigrationSummary",
    f"Ok({KEY}::ipc::migrate_legacy_identities(c.keys_mut(), identities)?)")

# ── signing session ──
cmd("begin_signing_session", "account_id: Option<String>", "()", "ops::begin_signing_session(c, account_id.as_deref())")
cmd("end_signing_session", "", "()", "{ ops::end_signing_session(c); Ok(()) }")
cmd("force_end_signing_session", "", "()", "{ ops::force_end_signing_session(c); Ok(()) }")
cmd("is_signing_session_active", "", "bool", "Ok(ops::is_signing_session_active(c))")

# ── preferences & logs ──
cmd("get_user_app_preferences", "", f"{SIG}::UserAppPreferences", "ops::get_user_app_preferences(c)")
cmd("set_user_app_preferences", f"preferences: {SIG}::UserAppPreferences", "()", "ops::set_user_app_preferences(c, preferences)")
cmd("reset_user_app_preferences", "", "()", "ops::reset_user_app_preferences(c)")
cmd("list_history_logs", "", f"Vec<{SIG}::logs::HistoryLog>", "Ok(ops::list_history_logs(c))")
cmd("list_activity_logs", "", f"Vec<{SIG}::logs::UserActivityLog>", "Ok(ops::list_activity_logs(c))")
cmd("clear_history_logs", "", "()", "ops::clear_history_logs(c)")
cmd("clear_activity_logs", "", "()", "ops::clear_activity_logs(c)")
cmd("restart_logs", "", "()", "ops::restart_logs(c)")
cmd("hydrate_logs", "", "()", "ops::hydrate_logs(c)")

# ── own account meta ──
cmd("update_own_account_meta", f"id: String, meta: {CON}::MajikContactMeta", "()", "ops::update_own_account_meta(c, &id, meta)")
cmd("resolve_signer_label", "signer_id: String", "String", "Ok(ops::resolve_signer_label(c, &signer_id))")

# ── contacts ──
SC = f"{CON}::SerializedMajikContact"
SG = f"{CON}::SerializedMajikContactGroup"
cmd("list_contacts", "sorted_by_label: Option<bool>, majikah_only: Option<bool>", f"Vec<{SC}>",
    "ops::list_contacts(c, sorted_by_label.unwrap_or(false), majikah_only.unwrap_or(false))")
cmd("get_contact", "id: String", f"Option<{SC}>", "ops::get_contact(c, &id)")
cmd("get_contact_by_fingerprint", "fingerprint: String", f"Option<{SC}>", "ops::get_contact_by_fingerprint(c, &fingerprint)")
cmd("get_contact_by_address", "address: String", f"Option<{SC}>", "ops::get_contact_by_address(c, &address)")
cmd("add_contact", f"contact: {SC}", "()", "ops::add_contact(c, contact)")
cmd("remove_contact", "id: String", "()", "ops::remove_contact(c, &id)")
cmd("update_contact_meta", f"id: String, meta: {CON}::MajikContactMeta", "()", "ops::update_contact_meta(c, &id, meta)")
cmd("import_contact_from_json", "json: String", "ApiResponseDto", "Ok(ops::import_contact_from_json(c, &json))")
cmd("import_contact_from_string", "data: String", "ApiResponseDto", "Ok(ops::import_contact_from_string(c, &data))")
cmd("export_contact_json", "id: String", "Option<String>", "ops::export_contact_json(c, &id)")
cmd("export_contact_string", "id: String", "Option<String>", "ops::export_contact_string(c, &id)")
cmd("clear_directory", "", "()", "ops::clear_directory(c)")
cmd("set_majikah_status", "id: String, status: bool", SC, "ops::set_majikah_status(c, &id, status)")
cmd("block_contact", "id: String", SC, "ops::block_contact(c, &id)")
cmd("unblock_contact", "id: String", SC, "ops::unblock_contact(c, &id)")

# ── groups ──
cmd("list_groups", "include_system: Option<bool>, sorted_by_name: Option<bool>", f"Vec<{SG}>",
    "ops::list_groups(c, include_system.unwrap_or(true), sorted_by_name.unwrap_or(false))")
cmd("get_group", "id: String", f"Option<{SG}>", "ops::get_group(c, &id)")
cmd("get_contacts_in_group", "id: String", f"Vec<{SC}>", "ops::get_contacts_in_group(c, &id)")
cmd("create_group", "args: CreateGroupArgs", SG, "ops::create_group(c, args)")
cmd("remove_group", "id: String", "ApiResponseDto", "ops::remove_group(c, &id)")
cmd("update_group_meta", "args: UpdateGroupMetaArgs", SG, "ops::update_group_meta(c, args)")
cmd("add_contact_to_group", "group_id: String, contact_id: String", SG, "ops::add_contact_to_group(c, &group_id, &contact_id)")
cmd("add_contacts_to_group", "group_id: String, contact_ids: Vec<String>", SG, "ops::add_contacts_to_group(c, &group_id, contact_ids)")
cmd("remove_contact_from_group", "group_id: String, contact_id: String", SG, "ops::remove_contact_from_group(c, &group_id, &contact_id)")
cmd("move_contact_between_groups", "contact_id: String, from_group_id: String, to_group_id: String", SG,
    "ops::move_contact_between_groups(c, &contact_id, &from_group_id, &to_group_id)")
cmd("add_contact_to_favorites", "contact_id: String", SG, "ops::add_contact_to_favorites(c, &contact_id)")
cmd("remove_contact_from_favorites", "contact_id: String", SG, "ops::remove_contact_from_favorites(c, &contact_id)")

# ── stamps ──
SM = f"{SIG}::StampMeta"
cmd("create_stamp", "args: CreateStampArgs", SM, "ops::create_stamp(c, args)")
cmd("list_stamps", f"kind: Option<{SIG}::StampAssetKind>", f"Vec<{SM}>", "ops::list_stamps(c, kind)")
cmd("list_stamps_for_active_account", "", f"Vec<{SM}>", "ops::list_stamps_for_active_account(c)")
cmd("hydrate_stamps", "", f"Vec<{SIG}::stamps::SkippedStamp>", "ops::hydrate_stamps(c)")
cmd("lock_stamps", "", "usize", "ops::lock_stamps(c)")
cmd("get_stamp", "id: String", f"Option<{SM}>", "ops::get_stamp(c, &id)")
cmd("get_decrypted_stamp", "id: String, output: Option<OutputTarget>", "Option<BytesOutput>",
    "ops::get_decrypted_stamp(c, &id, output.as_ref())")
cmd("decrypt_stamp_content", "id: String, account_id: Option<String>, output: Option<OutputTarget>", "BytesOutput",
    "ops::decrypt_stamp_content(c, &id, account_id.as_deref(), output.as_ref())")
cmd("rename_stamp", "id: String, name: String", SM, "ops::rename_stamp(c, &id, &name)")
cmd("replace_stamp_content", "id: String, input: FileInput, mime_type: Option<String>, account_id: Option<String>", SM,
    "ops::replace_stamp_content(c, &id, input, mime_type, account_id.as_deref())")
cmd("remove_stamp", "id: String", "()", "ops::remove_stamp(c, &id)")
cmd("inspect_stamp", "input: FileInput", "StampInspectDto", "ops::inspect_stamp(c, &input)")
cmd("is_stamped", "input: FileInput", "bool", "ops::is_stamped(c, &input)")

# ── content signing ──
VR = f"{SIG}::VerifyResult"
cmd("sign_text", "text: String, options: Option<SignTextOptionsDto>, ctx: Option<CtxDto>", "SignTextDto",
    "ops::sign_text(c, &text, options.unwrap_or_default(), ctx.unwrap_or_default())")
cmd("verify_text", "args: VerifyTextArgs", VR, "ops::verify_text(c, args)")
cmd("get_signature_metadata", "serialized: String", f"{SIG}::signing::SignatureMetadata", "ops::get_signature_metadata(c, &serialized)")

# ── file signing ──
cmd("sign_file", "args: SignFileArgs", "SignedFileDto", "ops::sign_file(c, args)")
cmd("sign_file_detached", "args: SignDetachedArgs", "DetachedSignedDto", "ops::sign_file_detached(c, args)")
cmd("resign_file", "args: ResignArgs", "SignedFileDto", "ops::resign_file(c, args)")
cmd("batch_sign_files", "args: BatchSignFilesArgs", "Vec<BatchSignOutcomeDto>", "ops::batch_sign_files(c, args)")

# ── two-step TSA ──
TS = f"{SGN}::MajikTimestamp"
cmd("begin_sign_file_with_tsa", "args: SignFileArgs", "TsaBeginDto", "ops::begin_sign_file_with_tsa(c, args)")
cmd("complete_sign_file_with_tsa", f"token: String, timestamp: {TS}, output: Option<OutputTarget>", "SignedFileDto",
    "ops::complete_sign_file_with_tsa(c, &token, timestamp, output.as_ref())")
cmd("begin_sign_file_detached_with_tsa", "args: SignDetachedArgs", "TsaBeginDto", "ops::begin_sign_file_detached_with_tsa(c, args)")
cmd("complete_sign_file_detached_with_tsa", f"token: String, timestamp: {TS}, output: Option<OutputTarget>", "DetachedSignedDto",
    "ops::complete_sign_file_detached_with_tsa(c, &token, timestamp, output.as_ref())")
cmd("cancel_pending_tsa", "token: String", "()", "{ ops::cancel_pending_tsa(c, &token); Ok(()) }")

# ── verify ──
cmd("verify_file", "args: VerifyFileArgs", VR, "ops::verify_file(c, args)")
cmd("verify_file_all_signatures", "args: VerifyFileArgs", f"Vec<{VR}>", "ops::verify_file_all_signatures(c, args)")
cmd("verify_file_detached", "args: VerifyDetachedArgs", VR, "ops::verify_file_detached(c, args)")
cmd("verify_file_detached_all_signatures", "args: VerifyDetachedArgs", f"Vec<{VR}>", "ops::verify_file_detached_all_signatures(c, args)")
cmd("verify_file_chain", "input: FileInput, now: Option<String>", "ChainDto", "ops::verify_file_chain(c, &input, now.as_deref())")
cmd("verify_file_detached_chain", "input: FileInput, envelope: FileInput, now: Option<String>", "ChainDto",
    "ops::verify_file_detached_chain(c, &input, &envelope, now.as_deref())")
cmd("verify_file_revisions", "final_file: FileInput, revisions: Vec<FileInput>", "RevisionSetDto", "ops::verify_file_revisions(c, &final_file, &revisions)")
cmd("verify_file_revisions_detached", "final_file: FileInput, revisions: Vec<FileInput>, envelope: FileInput", "RevisionSetDto",
    "ops::verify_file_revisions_detached(c, &final_file, &revisions, &envelope)")

# ── inspect / seal ──
cmd("extract_signatures", "input: FileInput", f"Vec<{SGN}::MajikSignatureJSON>", "ops::extract_signatures(c, &input)")
cmd("strip_signature", "input: FileInput, output: Option<OutputTarget>", "BytesOutput", "ops::strip_signature(c, &input, output.as_ref())")
cmd("is_file_signed", "input: FileInput", "bool", "ops::is_file_signed(c, &input)")
cmd("get_allowlist", "input: FileInput", f"Option<Vec<{SGN}::ExpectedSigner>>", "ops::get_allowlist(c, &input)")
cmd("can_sign", "input: FileInput, account_id: Option<String>", "PermitDto", "ops::can_sign(c, &input, account_id.as_deref())")
cmd("is_multi_sig", "input: FileInput", "bool", "ops::is_multi_sig(c, &input)")
cmd("get_signatories", f"input: FileInput, filter: Option<{SGN}::SignatoriesFilter>", f"Option<{SGN}::SignatoriesResult>", "ops::get_signatories(c, &input, filter)")
cmd("get_issuer", "input: FileInput", f"Option<{SGN}::SignatoryInfo>", "ops::get_issuer(c, &input)")
cmd("get_envelope_info", "input: FileInput", f"Option<{SGN}::EnvelopeInfo>", "ops::get_envelope_info(c, &input)")
cmd("get_file_signature_info", "input: FileInput", "FileSignatureInfoDto", "ops::get_file_signature_info(c, &input)")
cmd("seal", "input: FileInput, output: Option<OutputTarget>, ctx: Option<CtxDto>", "SealedFileDto",
    "ops::seal(c, &input, output.as_ref(), ctx.unwrap_or_default())")
cmd("verify_seal", "input: FileInput", f"{SGN}::SealVerificationResult", "ops::verify_seal(c, &input)")
cmd("get_seal_info", "input: FileInput", f"Option<{SGN}::SealInfo>", "ops::get_seal_info(c, &input)")
cmd("is_sealed", "input: FileInput", "bool", "ops::is_sealed(c, &input)")

# ── notary helpers ──
cmd("can_anchor", "input: FileInput", "PermitDto", "ops::can_anchor(&input)")
cmd("register_chain_anchor", f"input: FileInput, anchor: {SGN}::MajikChainAnchor, output: Option<OutputTarget>", "BytesOutput",
    "ops::register_chain_anchor(&input, anchor, output.as_ref())")

# ── maps & order ──
cmd("describe_mjks_map", "input: FileInput", "Vec<MapFileInfoDto>", "ops::describe_mjks_map(c, &input)")
cmd("is_mjks_map", "input: FileInput", "bool", "ops::is_mjks_map(c, &input)")
cmd("sign_batch_detached", "args: BatchDetachedArgs", "BatchDetachedDto", "ops::sign_batch_detached(c, args)")
cmd("cosign_mjks_map", "args: CosignArgs", "CosignDto", "ops::cosign_mjks_map(c, args)")
cmd("verify_mjks_map", "args: VerifyMapArgs", "MapVerifyDto", "ops::verify_mjks_map(c, args)")
cmd("verify_file_order", "args: OrderArgs", f"{SGN}::SignatureOrderResult", "ops::verify_file_order(c, args)")
cmd("verify_mjks_map_order",
    f"map: FileInput, files: Vec<FileInput>, expected: Vec<SignerRefDto>, strict: Option<bool>, source: Option<{SIG}::logs::HistorySource>",
    "Vec<MapOrderDto>",
    "ops::verify_mjks_map_order(c, &map, &files, &expected, strict.unwrap_or(false), source)")

# ── backups ──
cmd("backup_contacts", "output: Option<OutputTarget>", "BytesOutput", "ops::backup_contacts(c, output.as_ref())")
cmd("backup_stamps", "output: Option<OutputTarget>", "BytesOutput", "ops::backup_stamps(c, output.as_ref())")
cmd("backup_app_data", "output: Option<OutputTarget>", "BytesOutput", "ops::backup_app_data(c, output.as_ref())")
cmd("probe_backup_type", "input: FileInput", f"{SIG}::backup::BackupType", "ops::probe_backup_type(&input)")
cmd("read_app_data_backup", "input: FileInput", "AppDataSummaryDto", "ops::read_app_data_backup(c, &input)")
cmd("restore_contacts", "input: FileInput, overwrite_contacts: Option<bool>, include_groups: Option<bool>", f"{SIG}::backup::RestoredContacts",
    "ops::restore_contacts(c, &input, overwrite_contacts.unwrap_or(true), include_groups.unwrap_or(true))")
cmd("restore_stamps", "input: FileInput", "usize", "ops::restore_stamps(c, &input)")
cmd("restore_app_data", "input: FileInput", f"{SIG}::backup::RestoredAppData", "ops::restore_app_data(c, &input)")
cmd("restore_app_data_selective", "input: FileInput, options: Option<SelectiveRestoreDto>", f"{SIG}::backup::RestoredSelective",
    "ops::restore_app_data_selective(c, &input, options.unwrap_or(SelectiveRestoreDto { stamps: true, contacts: true, groups: true, preferences: true, overwrite_contacts: true }))")

names = [n for n,_,_,_ in C]
assert len(names) == len(set(names)), "duplicate command names"

# ---- commands.rs ----
out = ['''//! GENERATED by gen.py -- do not edit. Each command locks the client on the blocking pool and
//! calls one function in `majik_signature_client::ipc::ops` (or `majik_key_client::ipc`).

use majik_signature_client::ipc::ops::{self};
use majik_signature_client::ipc::*;
use majik_signature_client::SignatureClient;
use majik_signature_client::SignatureClientError;
use tauri::State;
use zeroize::Zeroizing;

use crate::handle::SigHandle;

type Handle<'a> = State<'a, SigHandle>;
type Res<T> = Result<T, SignatureClientError>;
''']
for name, params, ret, body in C:
    plist = (", " + params) if params else ""
    out.append(f"#[tauri::command]\n#[allow(unused_variables)]\npub async fn {name}(state: Handle<'_>{plist}) -> Res<{ret}> {{\n    state\n        .run(move |c: &mut SignatureClient| {body})\n        .await\n}}\n")
open("src/commands.rs","w").write("\n".join(out))

# ---- build.rs ----
open("build.rs","w").write("const COMMANDS: &[&str] = &[\n" + "".join(f'    "{n}",\n' for n in names) + "];\n\nfn main() {\n    tauri_plugin::Builder::new(COMMANDS).build();\n}\n")

# ---- permissions/default.toml ----
open("permissions/default.toml","w").write('[default]\ndescription = "Allows every majik-signature-client command."\npermissions = [\n' + "".join(f'    "allow-{n.replace("_","-")}",\n' for n in names) + "]\n")

# ---- lib.rs ----
open("src/lib.rs","w").write('''//! # tauri-plugin-majik-signature-client
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

use majik_signature_client::{SignatureClient, SharedConnection};
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, RunEvent, Runtime};

pub const PLUGIN_NAME: &str = "majik-signature-client";

/// `conn` is the app's shared connection; the plugin creates the key, state, contact, group,
/// stamp and log tables if missing. The client is hydrated by the first `open_session` call.
pub fn init<R: Runtime>(conn: SharedConnection) -> TauriPlugin<R> {
    Builder::<R>::new(PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
''' + "".join(f"            commands::{n},\n" for n in names) + '''        ])
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
''')
# ---- machine-readable command contract ----
Path("contract").mkdir(exist_ok=True)
contract = {
    "plugin": "majik-signature-client",
    "version": "1.2.0",
    "eventChannel": "majik-signature-client:event",
    "eventTypes": [
        "new-account",
        "removed-account",
        "updated-account",
        "active-account-change",
        "unlock",
        "lock",
        "error",
        "restore-backup",
        "sign",
        "verify",
        "new-stamp",
        "removed-stamp",
        "new-contact",
        "new-contact-group",
        "removed-contact",
        "removed-contact-group",
        "contact-group-change",
        "history-log",
        "activity-log",
    ],
    "commandCount": len(C),
    "commands": [
        {"name": name, "parameters": params, "result": ret}
        for name, params, ret, _body in C
    ],
}
Path("contract/commands.json").write_text(
    json.dumps(contract, indent=2, ensure_ascii=False) + "\n",
    encoding="utf-8",
)

print(len(names), "commands")
